//! 系統匣：Windows 工作列上「釘選」在外面的 app 圖示。
//!
//! 工作列被 Harbor 隱藏後，這些圖示只剩「顯示隱藏的圖示」箭頭能看到；
//! 頂部狀態列用這個模組把它們搬出來。
//!
//! 資料來源：
//! - 圖示清單、順序與圖片：Explorer 自己存的
//!   `HKCU\Control Panel\NotifyIconSettings`。`IsPromoted = 1` 代表釘選，
//!   `UIOrderList` 是工作列上的順序，`IconSnapshot` 是最近一次的圖示 PNG；
//!   再濾掉沒在執行的 app。不讀工作列本身：Harbor 讓工作列自動隱藏，
//!   收起時 UI Automation 讀不到系統匣。
//! - 點擊：送 Win+B 讓透明的工作列展開，再對那顆按鈕呼叫 UI Automation 的
//!   Invoke，app 收到的就跟使用者點系統匣一樣。
//!
//! 輸入法的中／英模式刻意不做：華碩智慧輸入法這類 TSF 輸入法用 Shift 切換時
//! 不回報給 IMM，跨行程讀語言列也被 Windows 擋下，顯示出來只會是錯的。
//!
//! 成本：一條背景執行緒，每 2 秒讀一次登錄檔與行程清單；內容沒變就不通知
//! 狀態列重繪。

use std::{
    collections::{HashMap, HashSet},
    sync::{
        Arc, Mutex,
        atomic::{AtomicBool, Ordering},
    },
    thread,
    time::{Duration, Instant},
};

use windows::{
    Win32::{
        Foundation::{CloseHandle, HWND, LPARAM, WPARAM},
        System::{
            Com::{
                CLSCTX_INPROC_SERVER, COINIT_MULTITHREADED, CoCreateInstance, CoInitializeEx,
                CoTaskMemFree, CoUninitialize,
            },
            ProcessStatus::EnumProcesses,
            Registry::{
                HKEY_CURRENT_USER, RRF_RT_REG_BINARY, RRF_RT_REG_DWORD, RRF_RT_REG_SZ, RegGetValueW,
            },
            Threading::{
                OpenProcess, PROCESS_NAME_WIN32, PROCESS_QUERY_LIMITED_INFORMATION,
                QueryFullProcessImageNameW,
            },
            Variant::VARIANT,
        },
        UI::{
            Accessibility::{
                CUIAutomation, IUIAutomation, IUIAutomationCondition, IUIAutomationElementArray,
                IUIAutomationInvokePattern, TreeScope_Descendants, UIA_AutomationIdPropertyId,
                UIA_InvokePatternId,
            },
            Shell::{KF_FLAG_DEFAULT, SHGetKnownFolderPath},
            WindowsAndMessaging::{
                FindWindowW, GetForegroundWindow, GetWindowThreadProcessId, PostMessageW,
            },
        },
    },
    core::{BSTR, GUID, HSTRING, PWSTR, w},
};

use crate::{icons::IconImage, imagery};

const SETTINGS_KEY: &str = r"Control Panel\NotifyIconSettings";
/// 工作列上釘選圖示按鈕的 AutomationId（Windows 11 的 XAML 系統匣）。
const PINNED_BUTTON_ID: &str = "NotifyItemIcon";
const TRAY_POLL: Duration = Duration::from_secs(2);
/// 送出 Win+B 後，等工作列展開、系統匣按鈕出現的上限。
const REVEAL_TIMEOUT: Duration = Duration::from_millis(1500);
/// 狀態列圖示 16 邏輯像素，最高 200% 縮放；IconSnapshot 本身也是 32×32。
const ICON_SIDE: u32 = 32;

/// 一顆釘選在外面的系統匣圖示。
///
/// 不含 tooltip：G-Helper 這類 app 每幾秒就改一次文字，比較它只會讓狀態列
/// 白白重繪。
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PinnedIcon {
    /// `NotifyIconSettings` 底下的子機碼。
    pub key: u64,
    pub icon: Option<Arc<IconImage>>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Snapshot {
    /// 由左到右，和 Windows 工作列的順序相同。
    pub pinned: Vec<PinnedIcon>,
}

/// 背景讀取系統匣；有變化時對 `window` 送出 `message`。
/// 丟棄時通知執行緒結束。
pub struct Watch {
    state: Arc<Mutex<Snapshot>>,
    stop: Arc<AtomicBool>,
}

impl Watch {
    #[must_use]
    pub fn start(window: isize, message: u32) -> Self {
        let state = Arc::new(Mutex::new(Snapshot::default()));
        let stop = Arc::new(AtomicBool::new(false));
        let (shared, stopped) = (Arc::clone(&state), Arc::clone(&stop));
        let spawned = thread::Builder::new()
            .name("harbor-tray".into())
            .spawn(move || run(window, message, &shared, &stopped));
        if let Err(error) = spawned {
            crate::log::line(&format!(
                "no se pudo iniciar el lector de la bandeja: {error}"
            ));
        }
        Self { state, stop }
    }

    #[must_use]
    pub fn snapshot(&self) -> Snapshot {
        self.state
            .lock()
            .map(|state| state.clone())
            .unwrap_or_default()
    }
}

impl Drop for Watch {
    fn drop(&mut self) {
        self.stop.store(true, Ordering::Relaxed);
    }
}

fn run(window: isize, message: u32, state: &Mutex<Snapshot>, stop: &AtomicBool) {
    // SAFETY: 這條執行緒自己的 COM apartment，結束前釋放。
    let initialized = unsafe { CoInitializeEx(None, COINIT_MULTITHREADED) }.is_ok();
    {
        let mut reader = TrayReader::new();
        let mut current = Snapshot::default();
        while !stop.load(Ordering::Relaxed) {
            let next = Snapshot {
                pinned: reader.read(),
            };
            if next != current {
                if let Ok(mut shared) = state.lock() {
                    shared.clone_from(&next);
                }
                current = next;
                // SAFETY: 只是把通知投遞到狀態列視窗；視窗已關閉時會失敗，無害。
                unsafe {
                    let _ =
                        PostMessageW(Some(HWND(window as *mut _)), message, WPARAM(0), LPARAM(0));
                }
            }
            thread::sleep(TRAY_POLL);
        }
        // `reader` 持有 COM 物件，必須在 CoUninitialize 之前釋放。
    }
    if initialized {
        // SAFETY: 與上面的 CoInitializeEx 成對。
        unsafe { CoUninitialize() };
    }
}

/// 左鍵點擊一顆釘選圖示，效果和點工作列系統匣相同。
///
/// Harbor 隱藏工作列時用的是「自動隱藏」，收起來的工作列不會把系統匣按鈕
/// 交給 UI Automation；所以找不到按鈕時先送 Win+B，讓（透明的）工作列
/// 展開、系統匣取得焦點，再找一次。這跟「顯示隱藏的圖示」箭頭用的是同一招。
///
/// `key` 是 `NotifyIconSettings` 的子機碼，用來比對 tooltip／執行檔名稱；
/// 比不到時退回用 `index`（狀態列上由左數第幾顆，順序與工作列相同）。
/// 跨行程的 UI Automation 呼叫可能要幾十毫秒，放在拋棄式執行緒。
pub fn activate(key: u64, index: usize) {
    let spawned = thread::Builder::new()
        .name("harbor-tray-click".into())
        .spawn(move || {
            let entry = Entry::read(key);
            // app 的面板或視窗已經在前景：這一下是要收起它。真正的工作列也是
            // 讓它失去焦點來收起；若照樣 Invoke，Win+B 先讓面板收起、Invoke
            // 又把它打開，看起來就是閃一下又出現。
            if entry.path.as_deref().is_some_and(is_foreground_process) {
                crate::launcher::focus_notification_area();
                return;
            }
            // SAFETY: 新執行緒自己的 apartment，結束前釋放。
            let initialized = unsafe { CoInitializeEx(None, COINIT_MULTITHREADED) }.is_ok();
            if let Some(tray) = TrayButtons::new() {
                let mut buttons = tray.find();
                if buttons.is_none() {
                    crate::launcher::focus_notification_area();
                    let until = Instant::now() + REVEAL_TIMEOUT;
                    while buttons.is_none() && Instant::now() < until {
                        thread::sleep(Duration::from_millis(50));
                        buttons = tray.find();
                    }
                }
                match buttons {
                    Some(buttons) => {
                        // 名稱與點擊用同一份清單：重查的話，工作列可能已經又收起，
                        // 清單變空，按索引取按鈕就會是無效參數（0x80070057）。
                        let names = button_names(&buttons);
                        let position = names
                            .iter()
                            .position(|name| entry.matches(name))
                            .or_else(|| (index < names.len()).then_some(index));
                        let result = position.map(|position| invoke(&buttons, position));
                        match result {
                            Some(Err(error)) => crate::log::line(&format!(
                                "no se pudo abrir el icono de la bandeja: {error}"
                            )),
                            None => crate::log::line(
                                "no se encontró el icono de la bandeja en la barra de tareas",
                            ),
                            Some(Ok(())) => {}
                        }
                    }
                    None => crate::log::line("la barra de tareas no mostró la bandeja a tiempo"),
                }
            }
            if initialized {
                // SAFETY: 與上面的 CoInitializeEx 成對。
                unsafe { CoUninitialize() };
            }
        });
    if let Err(error) = spawned {
        crate::log::line(&format!("no se pudo abrir el icono de la bandeja: {error}"));
    }
}

/// 前景視窗是否屬於 `path` 這個執行檔。
fn is_foreground_process(path: &str) -> bool {
    let mut pid = 0u32;
    // SAFETY: 唯讀查詢；`pid` 是有效的輸出參數。
    unsafe {
        let window = GetForegroundWindow();
        if window.is_invalid() {
            return false;
        }
        GetWindowThreadProcessId(window, Some(&mut pid));
    }
    process_image(pid).is_some_and(|image| image == path.to_lowercase())
}

/// 工作列上的釘選圖示按鈕（UI Automation）。只在點擊時使用。
struct TrayButtons {
    automation: IUIAutomation,
    condition: IUIAutomationCondition,
}

impl TrayButtons {
    /// Windows 10 的系統匣是舊式工具列，沒有這些 AutomationId。
    fn new() -> Option<Self> {
        if !crate::system::is_windows_11() {
            return None;
        }
        // SAFETY: 呼叫端已初始化 COM；`id` 在呼叫期間有效。
        unsafe {
            let automation: IUIAutomation =
                CoCreateInstance(&CUIAutomation, None, CLSCTX_INPROC_SERVER).ok()?;
            let id = VARIANT::from(BSTR::from(PINNED_BUTTON_ID));
            let condition = automation
                .CreatePropertyCondition(UIA_AutomationIdPropertyId, &id)
                .ok()?;
            Some(Self {
                automation,
                condition,
            })
        }
    }

    /// 由左到右的按鈕；工作列收起時（清單是空的）回傳 `None`。
    fn find(&self) -> Option<IUIAutomationElementArray> {
        // SAFETY: 唯讀的視窗查詢與 UI Automation 查詢。
        unsafe {
            // 每次重新找：Explorer 重啟後工作列是新的視窗。
            let tray = FindWindowW(w!("Shell_TrayWnd"), None).ok()?;
            let root = self.automation.ElementFromHandle(tray).ok()?;
            let buttons = root.FindAll(TreeScope_Descendants, &self.condition).ok()?;
            (buttons.Length().unwrap_or(0) > 0).then_some(buttons)
        }
    }
}

fn button_names(buttons: &IUIAutomationElementArray) -> Vec<String> {
    // SAFETY: 唯讀的 UI Automation 查詢。
    unsafe {
        let count = buttons.Length().unwrap_or(0).max(0);
        (0..count)
            .filter_map(|index| buttons.GetElement(index).ok())
            .map(|element| {
                element
                    .CurrentName()
                    .map(|name| name.to_string())
                    .unwrap_or_default()
            })
            .collect()
    }
}

fn invoke(buttons: &IUIAutomationElementArray, position: usize) -> windows::core::Result<()> {
    let index = i32::try_from(position).map_err(|_| windows::core::Error::empty())?;
    // SAFETY: 剛查到的工作列按鈕；Invoke 等同使用者點擊。
    unsafe {
        buttons
            .GetElement(index)?
            .GetCurrentPatternAs::<IUIAutomationInvokePattern>(UIA_InvokePatternId)?
            .Invoke()
    }
}

/// 從登錄檔讀釘選圖示；重複使用解碼過的圖示。
///
/// 不經過工作列：Harbor 讓工作列自動隱藏，收起時讀不到系統匣，之前因此
/// 只有點過箭頭（工作列展開）後才看得到圖示，一收起又消失。
struct TrayReader {
    /// 子機碼 → (PNG 原始位元組, 解碼後的圖示)。PNG 沒變就不重新解碼。
    icons: HashMap<u64, (Vec<u8>, Option<Arc<IconImage>>)>,
}

impl TrayReader {
    fn new() -> Self {
        Self {
            icons: HashMap::new(),
        }
    }

    /// 釘選（`IsPromoted = 1`）且正在執行的 app，照 `UIOrderList` 的順序，
    /// 也就是工作列上由左到右的順序。
    fn read(&mut self) -> Vec<PinnedIcon> {
        let promoted: Vec<u64> = ordered_keys()
            .into_iter()
            .filter(|&key| registry_dword(key, "IsPromoted") == Some(1))
            .collect();
        if promoted.is_empty() {
            return Vec::new();
        }
        let running = running_images();
        promoted
            .into_iter()
            .map(Entry::read)
            .filter(|entry| {
                entry
                    .path
                    .as_ref()
                    .is_some_and(|path| running.contains(&path.to_lowercase()))
            })
            .map(|entry| PinnedIcon {
                key: entry.key,
                icon: self.icon(entry.key),
            })
            .collect()
    }

    fn icon(&mut self, key: u64) -> Option<Arc<IconImage>> {
        let bytes = registry_bytes(&format!(r"{SETTINGS_KEY}\{key}"), "IconSnapshot")?;
        if let Some((cached, icon)) = self.icons.get(&key)
            && *cached == bytes
        {
            return icon.clone();
        }
        let icon = imagery::decode(&bytes, ICON_SIDE, 256 * 256).map(Arc::new);
        self.icons.insert(key, (bytes, icon.clone()));
        icon
    }
}

/// 釘選項目在登錄檔裡的資料。
struct Entry {
    key: u64,
    /// 執行檔的完整路徑；已知資料夾的 GUID 前綴已展開。
    path: Option<String>,
    tooltip: Option<String>,
}

impl Entry {
    fn read(key: u64) -> Self {
        let subkey = format!(r"{SETTINGS_KEY}\{key}");
        Self {
            key,
            path: registry_text(&subkey, "ExecutablePath").map(|path| expand_known_folder(&path)),
            tooltip: registry_text(&subkey, "InitialTooltip"),
        }
    }

    fn matches(&self, name: &str) -> bool {
        let first_line = |text: &str| text.lines().next().unwrap_or_default().trim().to_owned();
        if self
            .tooltip
            .as_deref()
            .is_some_and(|tooltip| !tooltip.is_empty() && first_line(tooltip) == first_line(name))
        {
            return true;
        }
        // 例如 EarTrumpet 的 tooltip 是「EarTrumpet: 10% - 喇叭」。
        self.path
            .as_deref()
            .and_then(|path| path.rsplit('\\').next())
            .and_then(|file| file.rsplit_once('.').map(|(stem, _)| stem.to_lowercase()))
            .is_some_and(|stem| stem.len() > 2 && name.to_lowercase().contains(&stem))
    }
}

/// `UIOrderList`：一串 little-endian 的 u64 子機碼，順序就是使用者排的順序。
fn ordered_keys() -> Vec<u64> {
    registry_bytes(SETTINGS_KEY, "UIOrderList")
        .unwrap_or_default()
        .as_chunks::<8>()
        .0
        .iter()
        .map(|chunk| u64::from_le_bytes(*chunk))
        .collect()
}

/// `{1AC14E77-...}\Taskmgr.exe` → `C:\Windows\System32\Taskmgr.exe`。
fn expand_known_folder(path: &str) -> String {
    let Some(rest) = path.strip_prefix('{') else {
        return path.to_owned();
    };
    let Some((id, tail)) = rest.split_once('}') else {
        return path.to_owned();
    };
    let Ok(folder) = GUID::try_from(id) else {
        return path.to_owned();
    };
    // SAFETY: 回傳的字串複製後，用配置它的記憶體配置器釋放。
    let base = unsafe {
        SHGetKnownFolderPath(&folder, KF_FLAG_DEFAULT, None)
            .ok()
            .map(|raw| {
                let text = raw.to_string().ok();
                CoTaskMemFree(Some(raw.as_ptr().cast()));
                text
            })
    };
    match base.flatten() {
        Some(base) => format!("{base}{tail}"),
        None => path.to_owned(),
    }
}

/// 目前執行中的行程的執行檔路徑（小寫）。系統行程打不開就略過。
fn running_images() -> HashSet<String> {
    let mut pids = vec![0u32; 2048];
    let mut needed = 0u32;
    // SAFETY: `pids` 的大小如實告知；Windows 最多寫滿它。
    let listed = unsafe {
        EnumProcesses(
            pids.as_mut_ptr(),
            (pids.len() * size_of::<u32>()) as u32,
            &mut needed,
        )
    };
    if listed.is_err() {
        return HashSet::new();
    }
    pids.truncate(needed as usize / size_of::<u32>());
    pids.into_iter().filter_map(process_image).collect()
}

/// 行程的執行檔完整路徑（小寫）；系統行程打不開時為 `None`。
fn process_image(pid: u32) -> Option<String> {
    // SAFETY: 以最小權限開啟，讀完立刻關閉。
    unsafe {
        let process = OpenProcess(PROCESS_QUERY_LIMITED_INFORMATION, false, pid).ok()?;
        let mut buffer = [0u16; 1024];
        let mut len = buffer.len() as u32;
        let read = QueryFullProcessImageNameW(
            process,
            PROCESS_NAME_WIN32,
            PWSTR(buffer.as_mut_ptr()),
            &mut len,
        );
        let _ = CloseHandle(process);
        read.ok()?;
        Some(String::from_utf16_lossy(buffer.get(..len as usize)?).to_lowercase())
    }
}

fn registry_dword(key: u64, value: &str) -> Option<u32> {
    let (subkey, value) = (
        HSTRING::from(format!(r"{SETTINGS_KEY}\{key}")),
        HSTRING::from(value),
    );
    let mut data = 0u32;
    let mut size = size_of::<u32>() as u32;
    // SAFETY: `data` 與 `size` 描述一個有效的 4 位元組緩衝區。
    unsafe {
        RegGetValueW(
            HKEY_CURRENT_USER,
            &subkey,
            &value,
            RRF_RT_REG_DWORD,
            None,
            Some((&raw mut data).cast()),
            Some(&mut size),
        )
    }
    .is_ok()
    .then_some(data)
}

fn registry_bytes(subkey: &str, value: &str) -> Option<Vec<u8>> {
    registry_raw(subkey, value, RRF_RT_REG_BINARY)
}

fn registry_text(subkey: &str, value: &str) -> Option<String> {
    let bytes = registry_raw(subkey, value, RRF_RT_REG_SZ)?;
    let wide: Vec<u16> = bytes
        .as_chunks::<2>()
        .0
        .iter()
        .map(|pair| u16::from_le_bytes(*pair))
        .take_while(|&unit| unit != 0)
        .collect();
    Some(String::from_utf16_lossy(&wide))
}

/// 先問大小再讀，避免截斷 IconSnapshot 這類長度不定的值。
fn registry_raw(
    subkey: &str,
    value: &str,
    kind: windows::Win32::System::Registry::REG_ROUTINE_FLAGS,
) -> Option<Vec<u8>> {
    let (subkey, value) = (HSTRING::from(subkey), HSTRING::from(value));
    let mut size = 0u32;
    // SAFETY: 只查詢大小，不寫入任何緩衝區。
    unsafe {
        RegGetValueW(
            HKEY_CURRENT_USER,
            &subkey,
            &value,
            kind,
            None,
            None,
            Some(&mut size),
        )
    }
    .ok()
    .ok()?;
    // 圖示快照只有幾 KB；過大代表資料異常，不讀。
    if size == 0 || size > 1024 * 1024 {
        return None;
    }
    let mut data = vec![0u8; size as usize];
    // SAFETY: `data` 與 `size` 描述同一個緩衝區。
    unsafe {
        RegGetValueW(
            HKEY_CURRENT_USER,
            &subkey,
            &value,
            kind,
            None,
            Some(data.as_mut_ptr().cast()),
            Some(&mut size),
        )
    }
    .ok()
    .ok()?;
    data.truncate(size as usize);
    Some(data)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn expands_known_folder_prefix() {
        let path = expand_known_folder(r"{F38BF404-1D43-42F2-9305-67DE0B28FC23}\explorer.exe");
        assert!(
            path.to_lowercase().ends_with(r"\windows\explorer.exe"),
            "{path}"
        );
    }

    #[test]
    fn leaves_plain_paths_alone() {
        assert_eq!(expand_known_folder(r"C:\a\b.exe"), r"C:\a\b.exe");
    }

    #[test]
    fn matches_tooltip_or_executable_name() {
        let entry = Entry {
            key: 1,
            path: Some(r"C:\Apps\EarTrumpet\EarTrumpet.exe".to_owned()),
            tooltip: Some("EarTrumpet: 36% - 喇叭".to_owned()),
        };
        assert!(entry.matches("EarTrumpet: 10% - 喇叭"));
        assert!(!entry.matches("CPU: 57°C"));
    }
}
