//! 暫時的診斷工具：用 DXGI Desktop Duplication 逐幀錄下主螢幕（每個 DWM
//! 合成幀都抓得到），同時在指定時間觸發一個動作，用來看神燈動畫哪一幀出錯。
//!
//! `spike_record list`
//! `spike_record <輸出檔> <錄幾毫秒> min <hwnd>`
//! `spike_record <輸出檔> <錄幾毫秒> click <x> <y>`
//!
//! 輸出檔：每幀一筆 [u64 微秒][u32 寬][u32 高][BGRA，縮成 1/4]。

use std::{fs::File, io::Write, time::Instant};

use anyhow::{Context, Result, anyhow};
use windows::{
    Win32::{
        Foundation::{HMODULE, HWND, LPARAM, RECT, WPARAM},
        Graphics::{
            Direct3D::D3D_DRIVER_TYPE_HARDWARE,
            Direct3D11::{
                D3D11_CPU_ACCESS_READ, D3D11_CREATE_DEVICE_BGRA_SUPPORT, D3D11_MAP_READ,
                D3D11_MAPPED_SUBRESOURCE, D3D11_SDK_VERSION, D3D11_TEXTURE2D_DESC,
                D3D11_USAGE_STAGING, D3D11CreateDevice, ID3D11Device, ID3D11DeviceContext,
                ID3D11Texture2D,
            },
            Dxgi::{
                DXGI_OUTDUPL_FRAME_INFO, IDXGIDevice, IDXGIOutput1, IDXGIResource,
            },
        },
        UI::{
            HiDpi::{DPI_AWARENESS_CONTEXT_PER_MONITOR_AWARE_V2, SetProcessDpiAwarenessContext},
            Input::KeyboardAndMouse::{
                INPUT, INPUT_0, INPUT_MOUSE, MOUSEEVENTF_LEFTDOWN, MOUSEEVENTF_LEFTUP, MOUSEINPUT,
                SendInput,
            },
            WindowsAndMessaging::{
                BringWindowToTop, EnumWindows, GetClassNameW, SetForegroundWindow, GetWindowRect, GetWindowTextW, IsIconic,
                IsWindowVisible, PostMessageW, SC_MINIMIZE, SetCursorPos, WM_SYSCOMMAND,
            },
        },
    },
    core::{BOOL, Interface},
};

const SCALE: usize = 4;

unsafe extern "system" fn each(hwnd: HWND, _: LPARAM) -> BOOL {
    // SAFETY: 唯讀查詢。
    unsafe {
        if IsWindowVisible(hwnd).as_bool() {
            let mut title = [0u16; 128];
            let mut class = [0u16; 128];
            let t = GetWindowTextW(hwnd, &mut title);
            let c = GetClassNameW(hwnd, &mut class);
            let mut rect = RECT::default();
            let _ = GetWindowRect(hwnd, &mut rect);
            if t > 0 {
                println!(
                    "{} iconic={} [{},{},{},{}] {} | {}",
                    hwnd.0 as isize,
                    IsIconic(hwnd).as_bool(),
                    rect.left,
                    rect.top,
                    rect.right,
                    rect.bottom,
                    String::from_utf16_lossy(&class[..c as usize]),
                    String::from_utf16_lossy(&title[..t as usize]),
                );
            }
        }
    }
    true.into()
}

fn click(x: i32, y: i32) {
    let input = |flags| INPUT {
        r#type: INPUT_MOUSE,
        Anonymous: INPUT_0 {
            mi: MOUSEINPUT {
                dwFlags: flags,
                ..Default::default()
            },
        },
    };
    // SAFETY: 合成一次左鍵點擊。
    unsafe {
        let _ = SetCursorPos(x, y);
        std::thread::sleep(std::time::Duration::from_millis(30));
        SendInput(
            &[input(MOUSEEVENTF_LEFTDOWN), input(MOUSEEVENTF_LEFTUP)],
            size_of::<INPUT>() as i32,
        );
    }
}

fn hit(hwnd: HWND, x: i32, y: i32) -> usize {
    use windows::Win32::UI::WindowsAndMessaging::{SMTO_ABORTIFHUNG, SendMessageTimeoutW, WM_NCHITTEST};
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

/// 找出最小化鈕，真實滑鼠輸入按下 100 ms 再放開。
fn press(hwnd: HWND) {
    let mut rect = RECT::default();
    // SAFETY: 唯讀查詢。
    unsafe {
        let _ = GetWindowRect(hwnd, &mut rect);
    }
    for dy in [16, 22, 28, 36, 44] {
        let hits: Vec<i32> = (0..100)
            .map(|k| rect.right - 10 - k * 4)
            .filter(|&x| hit(hwnd, x, rect.top + dy) == 8)
            .collect();
        if let (Some(&last), Some(&first)) = (hits.first(), hits.last()) {
            let (x, y) = ((first + last) / 2, rect.top + dy);
            let input = |flags| INPUT {
                r#type: INPUT_MOUSE,
                Anonymous: INPUT_0 {
                    mi: MOUSEINPUT {
                        dwFlags: flags,
                        ..Default::default()
                    },
                },
            };
            // SAFETY: 合成滑鼠輸入。
            unsafe {
                let _ = SetCursorPos(x, y);
                std::thread::sleep(std::time::Duration::from_millis(60));
                SendInput(&[input(MOUSEEVENTF_LEFTDOWN)], size_of::<INPUT>() as i32);
                std::thread::sleep(std::time::Duration::from_millis(100));
                SendInput(&[input(MOUSEEVENTF_LEFTUP)], size_of::<INPUT>() as i32);
            }
            return;
        }
    }
    println!("no min button");
}

fn main() -> Result<()> {
    // SAFETY: 行程開頭設定一次。
    unsafe {
        let _ = SetProcessDpiAwarenessContext(DPI_AWARENESS_CONTEXT_PER_MONITOR_AWARE_V2);
    }
    let args: Vec<String> = std::env::args().collect();
    if args.get(1).map(String::as_str) == Some("list") {
        // SAFETY: 回呼只做唯讀查詢。
        unsafe { EnumWindows(Some(each), LPARAM(0))? };
        return Ok(());
    }
    let out = args.get(1).context("輸出檔")?;
    let millis: u64 = args.get(2).context("毫秒")?.parse()?;
    let action = args.get(3).context("動作")?.clone();
    let a: i64 = args.get(4).context("參數")?.parse()?;
    let b: i64 = args.get(5).map_or(Ok(0), |v| v.parse())?;

    if action == "min" || action == "press" {
        // 先把要最小化的視窗帶到前景：按一下 Alt 讓前景鎖解除。
        // SAFETY: 合成 Alt 鍵與前景請求。
        unsafe {
            use windows::Win32::UI::Input::KeyboardAndMouse::{
                KEYBD_EVENT_FLAGS, KEYEVENTF_KEYUP, VK_MENU, keybd_event,
            };
            keybd_event(VK_MENU.0 as u8, 0, KEYBD_EVENT_FLAGS(0), 0);
            keybd_event(VK_MENU.0 as u8, 0, KEYEVENTF_KEYUP, 0);
            let hwnd = HWND(a as isize as *mut _);
            if IsIconic(hwnd).as_bool() {
                let _ = windows::Win32::UI::WindowsAndMessaging::ShowWindow(
                    hwnd,
                    windows::Win32::UI::WindowsAndMessaging::SW_RESTORE,
                );
            }
            windows::Win32::UI::WindowsAndMessaging::SwitchToThisWindow(hwnd, true);
            // 前景鎖：暫時把自己的輸入佇列接到目前前景視窗的執行緒上。
            {
                use windows::Win32::System::Threading::{AttachThreadInput, GetCurrentThreadId};
                use windows::Win32::UI::WindowsAndMessaging::{GetForegroundWindow, GetWindowThreadProcessId};
                let front = GetForegroundWindow();
                let theirs = GetWindowThreadProcessId(front, None);
                let ours = GetCurrentThreadId();
                let _ = AttachThreadInput(ours, theirs, true);
                let _ = SetForegroundWindow(hwnd);
                let _ = BringWindowToTop(hwnd);
                let _ = AttachThreadInput(ours, theirs, false);
            }
            let _ = SetForegroundWindow(hwnd);
            let _ = BringWindowToTop(hwnd);
            std::thread::sleep(std::time::Duration::from_millis(300));
            println!(
                "foreground ok = {}",
                windows::Win32::UI::WindowsAndMessaging::GetForegroundWindow() == hwnd
            );
        }
        std::thread::sleep(std::time::Duration::from_millis(500));
    }
    // SAFETY: 標準 D3D11／DXGI 初始化，所有 COM 物件由 windows crate 釋放。
    unsafe {
        let mut device: Option<ID3D11Device> = None;
        let mut context: Option<ID3D11DeviceContext> = None;
        D3D11CreateDevice(
            None,
            D3D_DRIVER_TYPE_HARDWARE,
            HMODULE::default(),
            D3D11_CREATE_DEVICE_BGRA_SUPPORT,
            None,
            D3D11_SDK_VERSION,
            Some(&mut device),
            None,
            Some(&mut context),
        )?;
        let device = device.ok_or_else(|| anyhow!("no device"))?;
        let context = context.ok_or_else(|| anyhow!("no context"))?;
        let output = device
            .cast::<IDXGIDevice>()?
            .GetAdapter()?
            .EnumOutputs(0)?
            .cast::<IDXGIOutput1>()?;
        let mut duplication = output.DuplicateOutput(&device)?;
        let mut file = File::create(out)?;
        let begin = Instant::now();
        let q = { let t = windows::Win32::System::SystemInformation::GetSystemTimePreciseAsFileTime(); (u64::from(t.dwHighDateTime) << 32 | u64::from(t.dwLowDateTime)) / 10 };
        println!("qpc begin={q}");
        let mut staging: Option<ID3D11Texture2D> = None;
        let mut fired = false;
        let mut frames = 0;
        while begin.elapsed().as_millis() < u128::from(millis) {
            if !fired && begin.elapsed().as_millis() >= 300 {
                fired = true;
                let at = begin.elapsed().as_micros();
                match action.as_str() {
                    "min" => {
                        let _ = PostMessageW(
                            Some(HWND(a as isize as *mut _)),
                            WM_SYSCOMMAND,
                            WPARAM(SC_MINIMIZE as usize),
                            LPARAM(0),
                        );
                    }
                    "click" => click(a as i32, b as i32),
                    "press" => press(HWND(a as isize as *mut _)),
                    "dock" => {
                        // 先停在底邊讓自動隱藏的 Dock 出來，再移到圖示上點擊。
                        let _ = SetCursorPos(a as i32, 1599);
                        std::thread::sleep(std::time::Duration::from_millis(700));
                        let _ = SetCursorPos(a as i32, b as i32);
                        std::thread::sleep(std::time::Duration::from_millis(250));
                        click(a as i32, b as i32);
                    }
                    "move" => {
                        let _ = SetCursorPos(a as i32, b as i32);
                    }
                    _ => {}
                }
                println!("action at {at} us");
            }
            let mut info = DXGI_OUTDUPL_FRAME_INFO::default();
            let mut resource: Option<IDXGIResource> = None;
            if let Err(error) = duplication.AcquireNextFrame(20, &mut info, &mut resource) {
                // 前景切換等狀況會讓複製失效（ACCESS_LOST），重建一個。
                if error.code() != windows::Win32::Graphics::Dxgi::DXGI_ERROR_WAIT_TIMEOUT {
                    println!("lost {:?} at {} ms", error.code(), begin.elapsed().as_millis());
                    std::thread::sleep(std::time::Duration::from_millis(2));
                    if let Ok(again) = output.DuplicateOutput(&device) {
                        duplication = again;
                    }
                }
                continue;
            }
            let stamp = begin.elapsed().as_micros() as u64;
            let Some(resource) = resource else {
                duplication.ReleaseFrame()?;
                continue;
            };
            let texture: ID3D11Texture2D = resource.cast()?;
            if staging.is_none() {
                let mut desc = D3D11_TEXTURE2D_DESC::default();
                texture.GetDesc(&mut desc);
                desc.Usage = D3D11_USAGE_STAGING;
                desc.BindFlags = 0;
                desc.CPUAccessFlags = D3D11_CPU_ACCESS_READ.0 as u32;
                desc.MiscFlags = 0;
                let mut created = None;
                device.CreateTexture2D(&desc, None, Some(&mut created))?;
                staging = created;
            }
            let staging_ref = staging.as_ref().ok_or_else(|| anyhow!("staging"))?;
            context.CopyResource(staging_ref, &texture);
            duplication.ReleaseFrame()?;
            let mut desc = D3D11_TEXTURE2D_DESC::default();
            staging_ref.GetDesc(&mut desc);
            let mut mapped = D3D11_MAPPED_SUBRESOURCE::default();
            context.Map(staging_ref, 0, D3D11_MAP_READ, 0, Some(&mut mapped))?;
            let (w, h) = (desc.Width as usize, desc.Height as usize);
            // CROP=l,t,r,b：只存這塊、全解析度。
            let crop: Option<Vec<usize>> = std::env::var("CROP")
                .ok()
                .map(|v| v.split(',').filter_map(|n| n.parse().ok()).collect());
            let (x0, y0, step, sw, sh) = match &crop {
                Some(c) if c.len() == 4 => (c[0], c[1], 1, c[2] - c[0], c[3] - c[1]),
                _ => (0, 0, SCALE, w / SCALE, h / SCALE),
            };
            let mut small = Vec::with_capacity(sw * sh * 4);
            let base = mapped.pData as *const u8;
            for y in 0..sh {
                let row = base.add((y0 + y * step) * mapped.RowPitch as usize);
                for x in 0..sw {
                    let px = std::slice::from_raw_parts(row.add((x0 + x * step) * 4), 4);
                    small.extend_from_slice(px);
                }
            }
            context.Unmap(staging_ref, 0);
            file.write_all(&stamp.to_le_bytes())?;
            file.write_all(&(sw as u32).to_le_bytes())?;
            file.write_all(&(sh as u32).to_le_bytes())?;
            file.write_all(&small)?;
            frames += 1;
        }
        println!("{frames} frames");
    }
    Ok(())
}
