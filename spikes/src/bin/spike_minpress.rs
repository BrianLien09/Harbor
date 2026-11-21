//! 暫時的診斷工具：按下視窗的最小化鈕（真實滑鼠輸入），記錄過程中的
//! WinEvent（滑鼠擷取開始／結束、最小化開始）與時間，看按下到最小化之間
//! 有沒有可以提前準備的訊號。
//! `spike_minpress <hwnd> [按住毫秒]`

use std::time::{Duration, Instant};

use anyhow::{Context, Result};
use windows::Win32::{
    Foundation::{HWND, LPARAM, POINT, RECT, WPARAM},
    UI::{
        Accessibility::{HWINEVENTHOOK, SetWinEventHook},
        HiDpi::{DPI_AWARENESS_CONTEXT_PER_MONITOR_AWARE_V2, SetProcessDpiAwarenessContext},
        Input::KeyboardAndMouse::{
            INPUT, INPUT_0, INPUT_MOUSE, KEYBD_EVENT_FLAGS, KEYEVENTF_KEYUP, MOUSEEVENTF_LEFTDOWN,
            MOUSEEVENTF_LEFTUP, MOUSEINPUT, SendInput, VK_MENU, keybd_event,
        },
        WindowsAndMessaging::{
            DispatchMessageW, EVENT_SYSTEM_CAPTUREEND, EVENT_SYSTEM_CAPTURESTART,
            EVENT_SYSTEM_MINIMIZESTART, EVENT_SYSTEM_MOVESIZESTART, GA_ROOT, GetAncestor,
            GetClassNameW, GetCursorPos, GetWindowRect, HTMINBUTTON, MSG, PM_REMOVE, PeekMessageW,
            SMTO_ABORTIFHUNG, SendMessageTimeoutW, SetCursorPos, SetForegroundWindow,
            SwitchToThisWindow, WINEVENT_OUTOFCONTEXT, WM_NCHITTEST,
        },
    },
};

static mut BEGIN: Option<Instant> = None;

fn hit(hwnd: HWND, x: i32, y: i32) -> usize {
    let mut result = 0usize;
    // SAFETY: 跨行程查詢，逾時就放棄。
    unsafe {
        let _ = SendMessageTimeoutW(
            hwnd,
            WM_NCHITTEST,
            WPARAM(0),
            LPARAM(((y as u32 as isize) << 16) | (x as u16 as isize)),
            SMTO_ABORTIFHUNG,
            50,
            Some(&mut result),
        );
    }
    result
}

unsafe extern "system" fn on_event(
    _: HWINEVENTHOOK,
    event: u32,
    hwnd: HWND,
    object: i32,
    _c: i32,
    _: u32,
    _: u32,
) {
    // SAFETY: 唯讀查詢；BEGIN 只在主執行緒寫一次。
    unsafe {
        let at = BEGIN.map_or(0.0, |b| b.elapsed().as_secs_f64() * 1000.0);
        let root = GetAncestor(hwnd, GA_ROOT);
        let mut class = [0u16; 64];
        let n = GetClassNameW(root, &mut class) as usize;
        let mut cursor = POINT::default();
        let _ = GetCursorPos(&mut cursor);
        let name = match event {
            EVENT_SYSTEM_CAPTURESTART => "CAPTURESTART",
            EVENT_SYSTEM_CAPTUREEND => "CAPTUREEND",
            EVENT_SYSTEM_MINIMIZESTART => "MINIMIZESTART",
            EVENT_SYSTEM_MOVESIZESTART => "MOVESIZESTART",
            _ => "?",
        };
        if event == 0x800B || event == 0x800C || event == 0x8005 || event == 0x8004 {
            return;
        }
        let ht = if event == EVENT_SYSTEM_CAPTURESTART {
            hit(root, cursor.x, cursor.y)
        } else {
            0
        };
        println!(
            "{at:8.1} ms {name} {event:#x} obj={object} child={_c} hwnd={} root={} {} hit={ht}",
            hwnd.0 as isize,
            root.0 as isize,
            String::from_utf16_lossy(&class[..n])
        );
    }
}

fn mouse(flags: windows::Win32::UI::Input::KeyboardAndMouse::MOUSE_EVENT_FLAGS) {
    let input = INPUT {
        r#type: INPUT_MOUSE,
        Anonymous: INPUT_0 {
            mi: MOUSEINPUT {
                dwFlags: flags,
                ..Default::default()
            },
        },
    };
    // SAFETY: 合成一個滑鼠事件。
    unsafe {
        SendInput(&[input], size_of::<INPUT>() as i32);
    }
}

fn pump(until: Instant) {
    let mut msg = MSG::default();
    while Instant::now() < until {
        // SAFETY: 本執行緒的訊息迴圈，WinEvent 回呼在這裡送達。
        unsafe {
            while PeekMessageW(&mut msg, None, 0, 0, PM_REMOVE).as_bool() {
                DispatchMessageW(&msg);
            }
        }
        std::thread::sleep(Duration::from_millis(1));
    }
}

fn main() -> Result<()> {
    // SAFETY: 行程開頭設定一次。
    unsafe {
        let _ = SetProcessDpiAwarenessContext(DPI_AWARENESS_CONTEXT_PER_MONITOR_AWARE_V2);
    }
    let args: Vec<String> = std::env::args().collect();
    let hwnd = HWND(args.get(1).context("hwnd")?.parse::<isize>()? as *mut _);
    let hold: u64 = args.get(2).map_or(Ok(100), |v| v.parse())?;
    // SAFETY: 前景切換與事件掛鉤，都是標準呼叫。
    unsafe {
        keybd_event(VK_MENU.0 as u8, 0, KEYBD_EVENT_FLAGS(0), 0);
        keybd_event(VK_MENU.0 as u8, 0, KEYEVENTF_KEYUP, 0);
        SwitchToThisWindow(hwnd, true);
        let _ = SetForegroundWindow(hwnd);
    }
    pump(Instant::now() + Duration::from_millis(400));
    let mut rect = RECT::default();
    // SAFETY: 唯讀查詢。
    unsafe { GetWindowRect(hwnd, &mut rect)? };
    // 沿標題列由右往左找最小化鈕。
    let mut target = None;
    'scan: for dy in [16, 22, 28, 36, 44] {
        let hits: Vec<i32> = (0..100)
            .map(|k| rect.right - 10 - k * 4)
            .filter(|&x| hit(hwnd, x, rect.top + dy) == HTMINBUTTON as usize)
            .collect();
        if let (Some(&last), Some(&first)) = (hits.first(), hits.last()) {
            target = Some(((first + last) / 2, rect.top + dy));
            break 'scan;
        }
    }
    let (x, y) = target.context("找不到最小化鈕")?;
    println!("min button at {x},{y}");
    // SAFETY: 掛鉤在這個執行緒，訊息迴圈在 pump。
    unsafe {
        SetWinEventHook(
            if std::env::var("ALL").is_ok() {
                1
            } else {
                EVENT_SYSTEM_CAPTURESTART
            },
            if std::env::var("ALL").is_ok() {
                0x7FFF_FFFF
            } else {
                EVENT_SYSTEM_MINIMIZESTART
            },
            None,
            Some(on_event),
            0,
            0,
            WINEVENT_OUTOFCONTEXT,
        );
        let _ = SetCursorPos(x, y);
        BEGIN = Some(Instant::now());
    }
    pump(Instant::now() + Duration::from_millis(150));
    println!("down");
    mouse(MOUSEEVENTF_LEFTDOWN);
    pump(Instant::now() + Duration::from_millis(hold));
    println!("up");
    mouse(MOUSEEVENTF_LEFTUP);
    pump(Instant::now() + Duration::from_millis(500));
    Ok(())
}
