//! Genie animation: a window flies into the dock when minimized and back out
//! when restored from the dock.
//!
//! Windows cannot deform another app's window, Windows 11 ignores the shell's
//! minimize-target hint, and a minimized window can no longer be captured. So,
//! to minimize:
//!
//! 1. one live DWM thumbnail is shown exactly where the window was (nothing
//!    changes on screen);
//! 2. at the same moment a worker thread copies that rectangle from the
//!    screen: a snapshot, as macOS also uses for its genie;
//! 3. the snapshot becomes one composition surface shared by thin horizontal
//!    strips (plain composition visuals) that follow the genie curves
//!    (`harbor_core::genie`), and they start flying at once;
//! 4. the stand-in stays under them for two more frames: composition commits
//!    asynchronously while DWM drops a thumbnail on the very next frame, and
//!    removing it at once left a frame with neither, a visible flash.
//!
//! The stand-in goes up before the dock even knows where the window will land
//! (`Flight::start`, then `Flight::aim`): the window has already vanished, and
//! every millisecond until the stand-in shows is a flash of the desktop.
//!
//! Until the strips move, the window just sits there, and that pause reads as
//! lag. It is the screen copy, 30-80 ms that GDI takes whatever the size, so
//! nothing else waits on it: no frames are counted before it, and none after.
//! Waiting two frames on each side used to add up to about 200 ms.
//!
//! The dock does not move the strips itself. Each strip gets two expression
//! animations (offset and scale) that read two shared values, `bend` and
//! `shift`, and those two are keyframe animations: the system compositor runs
//! the whole flight on its own thread at the display's refresh rate. Placing
//! ~300 strips from the UI thread every frame was measured at 8-10 ms a frame
//! and made the compositor skip every other refresh.
//!
//! To restore, the same strips play backwards from a quarter-size copy of that
//! snapshot (a [`Snapshot`], kept while the window is minimized), and the real
//! window is restored underneath once they reach full size. A window that was
//! not minimized with the genie has no snapshot and restores without it.
//!
//! Many DWM thumbnails instead of a snapshot were measured and rejected: DWM
//! drops frames beyond about 16 of them (plan section 8).
//!
//! On a dock at the left or right edge the genie runs the same way, in a
//! frame turned a quarter ([`Turn`]): the window flows "down" into the slot
//! in that frame, which the overlay shows turned toward the dock. The copy
//! of the window is turned to match on the capture worker. A pure rotation,
//! never a mirror, so its pixels stay as they were.
//!
//! Everything lives on a transparent overlay window that exists only while
//! the animation runs. The system's own minimize animation must already be
//! off for the window (see `App::calm_windows`): it starts before the dock
//! hears about the minimize and cannot be cancelled afterwards.

use std::{
    sync::{
        Arc, OnceLock,
        atomic::{AtomicBool, Ordering},
        mpsc::{Receiver, TryRecvError, channel},
    },
    thread,
    time::{Duration, Instant},
};

use anyhow::Result;
use harbor_core::{
    frame::DockEdge,
    genie::{Edges, Genie},
};
use harbor_shell::{icons::IconImage, screen, tracker};
use windows::{
    Foundation::{TimeSpan, TypedEventHandler},
    UI::Composition::{
        CompositionBatchTypes, CompositionPropertySet, CompositionStretch, ContainerVisual,
        Desktop::DesktopWindowTarget, SpriteVisual,
    },
    Win32::{
        Foundation::{HWND, LPARAM, LRESULT, RECT, WPARAM},
        Graphics::Dwm::{
            DWM_THUMBNAIL_PROPERTIES, DWM_TNP_OPACITY, DWM_TNP_RECTDESTINATION,
            DWM_TNP_SOURCECLIENTAREAONLY, DWM_TNP_VISIBLE, DwmFlush, DwmRegisterThumbnail,
            DwmUnregisterThumbnail, DwmUpdateThumbnailProperties,
        },
        System::{LibraryLoader::GetModuleHandleW, WinRT::Composition::ICompositorDesktopInterop},
        UI::WindowsAndMessaging::{
            CreateWindowExW, DefWindowProcW, DestroyWindow, HWND_TOPMOST, IsIconic, RegisterClassW,
            SW_SHOWNOACTIVATE, SWP_NOACTIVATE, SWP_NOMOVE, SWP_NOSIZE, SWP_SHOWWINDOW,
            SetWindowPos, ShowWindow, WNDCLASSW, WS_EX_LAYERED, WS_EX_NOACTIVATE,
            WS_EX_NOREDIRECTIONBITMAP, WS_EX_TOOLWINDOW, WS_EX_TOPMOST, WS_EX_TRANSPARENT,
            WS_POPUP,
        },
    },
    core::{HSTRING, Interface, h, w},
};
use windows_numerics::{Matrix3x2, Matrix4x4, Vector2};

use crate::gfx::Gfx;

/// Screen rows per strip: the edge of the funnel steps by at most a few pixels.
const STRIP_ROWS: u32 = 3;
const MAX_STRIPS: u32 = 400;
/// Samples of the bend and slide curves handed to the compositor, which
/// interpolates between them at the display's refresh rate.
const KEYFRAMES: u32 = 30;
/// Extra source rows each strip draws below itself. Strips sit at fractional
/// positions while they fly; overlapping the next one (same pixels) leaves no
/// seam between their antialiased edges.
const OVERLAP_ROWS: f32 = 1.0;
/// Frames a minimize waits to learn its dock slot before giving up.
const FRAMES_TO_AIM: u32 = 2;
/// Frames the strips and the stand-in overlap before the stand-in goes.
const HANDOVER_FRAMES: u32 = 2;
/// The dock's own thumbnail fades in over the last part of the animation.
const LANDING_START: f32 = 0.8;
/// How long the last frame of a restore stays up, so the real window has
/// certainly appeared underneath it.
const SETTLE: Duration = Duration::from_millis(120);
/// Abandon a stalled screen copy instead of keeping its overlay and frame loop alive.
const CAPTURE_TIMEOUT: Duration = Duration::from_millis(750);
const FLIGHT_TIMEOUT: Duration = Duration::from_secs(2);
/// 合成器動畫比 UI 時鐘多等的上限。負載高時動畫提交會晚於 UI 開始計時，
/// 正常情況由 `CompositionScopedBatch` 的完成事件收尾；萬一事件沒來
/// （裝置遺失等）就靠這個上限放行，不讓覆蓋層卡住。
const COMPLETION_GRACE: Duration = Duration::from_millis(400);
static CAPTURE_BUSY: AtomicBool = AtomicBool::new(false);

struct CapturePermit;

impl CapturePermit {
    fn acquire() -> Option<Self> {
        CAPTURE_BUSY
            .compare_exchange(false, true, Ordering::AcqRel, Ordering::Acquire)
            .ok()
            .map(|_| Self)
    }
}

impl Drop for CapturePermit {
    fn drop(&mut self) {
        CAPTURE_BUSY.store(false, Ordering::Release);
    }
}

/// What a minimized window looked like, to fly it back out of the dock.
/// Half the resolution in each direction: a quarter of the memory, and the
/// softness does not show in a half-second animation.
pub struct Snapshot {
    /// Turned for the dock's edge at the time (see [`Turn`]).
    image: IconImage,
    /// Where the window was, in screen coordinates.
    window: RECT,
    edge: DockEdge,
}

impl Snapshot {
    /// Width of the window divided by its height.
    pub fn aspect(&self) -> f32 {
        let window = self.window;
        (window.right - window.left) as f32 / (window.bottom - window.top).max(1) as f32
    }
}

/// The genie's frame on a dock at the left or right edge: the overlay
/// (`width` x `height` screen pixels) turned a quarter, so the window flows
/// toward the dock as it flows down toward a bottom dock.
#[derive(Clone, Copy, Debug, PartialEq)]
struct Turn {
    edge: DockEdge,
    width: f32,
    height: f32,
}

impl Turn {
    fn new(edge: DockEdge, screen: RECT) -> Self {
        Self {
            edge,
            width: (screen.right - screen.left) as f32,
            height: (screen.bottom - screen.top) as f32,
        }
    }

    /// The frame's size: the overlay's, turned on the sides.
    fn size(self) -> (f32, f32) {
        if self.edge.is_vertical() {
            (self.height, self.width)
        } else {
            (self.width, self.height)
        }
    }

    /// Overlay-local edges in the frame.
    fn edges(self, local: Edges) -> Edges {
        match self.edge {
            DockEdge::Bottom => local,
            // Frame x runs down the screen, frame y toward the left edge.
            DockEdge::Left => Edges {
                left: local.top,
                right: local.bottom,
                top: self.width - local.right,
                bottom: self.width - local.left,
            },
            // Frame x runs up the screen, frame y toward the right edge.
            DockEdge::Right => Edges {
                left: self.height - local.bottom,
                right: self.height - local.top,
                top: local.left,
                bottom: local.right,
            },
        }
    }

    /// The frame shown in the overlay (row vectors, as Composition).
    fn matrix(self) -> Option<Matrix4x4> {
        let (m11, m12, m21, m22, m41, m42) = match self.edge {
            DockEdge::Bottom => return None,
            DockEdge::Left => (0.0, 1.0, -1.0, 0.0, self.width, 0.0),
            DockEdge::Right => (0.0, -1.0, 1.0, 0.0, 0.0, self.height),
        };
        Some(Matrix4x4 {
            M11: m11,
            M12: m12,
            M21: m21,
            M22: m22,
            M33: 1.0,
            M41: m41,
            M42: m42,
            M44: 1.0,
            ..Matrix4x4::default()
        })
    }

    /// Turns the overlay's root into the frame. Its size becomes the
    /// frame's, or the clip at the slot would cut the turned strips off.
    fn apply(self, root: &ContainerVisual) -> Result<()> {
        if let Some(matrix) = self.matrix() {
            let (width, height) = self.size();
            root.SetRelativeSizeAdjustment(Vector2 { X: 0.0, Y: 0.0 })?;
            root.SetSize(Vector2 {
                X: width,
                Y: height,
            })?;
            root.SetTransformMatrix(matrix)?;
        }
        Ok(())
    }

    /// A copy of the window as it reads in the frame.
    fn image(self, image: IconImage) -> IconImage {
        if !self.edge.is_vertical() {
            return image;
        }
        let (w, h) = (image.width as usize, image.height as usize);
        let mut bgra = vec![0u8; w * h * 4];
        // In the frame the image is `h` wide and `w` tall.
        for fy in 0..w {
            for fx in 0..h {
                let (x, y) = match self.edge {
                    DockEdge::Left => (w - 1 - fy, fx),
                    _ => (fy, h - 1 - fx),
                };
                let from = (y * w + x) * 4;
                let to = (fy * h + fx) * 4;
                bgra[to..to + 4].copy_from_slice(&image.bgra[from..from + 4]);
            }
        }
        IconImage {
            width: image.height,
            height: image.width,
            bgra,
        }
    }
}

struct Piece {
    visual: SpriteVisual,
    /// First source row and row count, in snapshot pixels.
    top: f32,
    rows: f32,
}

struct Captured {
    image: IconImage,
    /// 還原用的縮小版，擷取執行緒在送出 `image` 之後才算（見 `Flight::start`）。
    restore: Receiver<IconImage>,
}

enum Phase {
    /// The stand-in thumbnail is on screen while a worker copies the screen.
    Standing {
        thumbnail: isize,
        frames: u32,
        shot: Receiver<Option<Captured>>,
        started: Instant,
    },
    Flying {
        start: Instant,
        /// Kept alive: the strips' animations read it.
        _drive: CompositionPropertySet,
        /// 合成器真的播完 `bend` 與 `shift` 之後才會變成 `true`。
        done: Arc<AtomicBool>,
        /// The stand-in of a minimize and the frames it has overlapped the
        /// strips, until they are certainly on screen.
        stand_in: Option<(isize, u32)>,
    },
    /// A restore has reached full size and waits for the real window.
    Settling { until: Instant },
}

/// 下一次最小化要用的替身與動畫視窗，事先備好。
///
/// 系統轉場關掉後，視窗在按下最小化後的下一個合成幀就消失。替身若等事件
/// 來了才建（兩個視窗、合成目標、註冊縮圖、顯示，debug build 共 15-18 ms），
/// 常常晚一幀才上畫面，那一幀露出後面的桌面，就是「先閃一下」。
///
/// 所以平常就備好：替身視窗一直顯示著（透明、點擊穿透、沒有內容），前景
/// 視窗的縮圖也先註冊好、擺好位置但不顯示；事件來時只剩一次
/// `DwmUpdateThumbnailProperties` 加 `DwmFlush`。實測這樣仍有少數幾次會晚
/// 一幀：DWM 套用縮圖變更的時機本來就可能比視窗隱藏晚。
///
/// 能預知的最小化就不靠搶時間：按下標題列最小化鈕的那一刻、或 dock 自己
/// 要最小化前景 app 時，先用 [`Spare::arm_behind`] 把縮圖顯示在視窗正後方
/// （被視窗本身蓋住，看不到）。視窗一消失，替身在同一幀就露出來。
///
/// 動畫視窗（overlay）維持隱藏，等替身上去之後才顯示。全螢幕時 dock 不播
/// 神燈，`App::prepare_spare` 會拆掉這組視窗，避免壓在全螢幕程式上。
pub struct Spare {
    screen: RECT,
    parts: Option<Parts>,
}

/// 預先註冊好的縮圖。
struct Armed {
    hwnd: isize,
    thumbnail: isize,
    /// 縮圖擺放的位置（螢幕座標），和最小化時的位置不同就得重設。
    from: RECT,
    shown: bool,
}

struct Parts {
    stand: HWND,
    overlay: HWND,
    target: DesktopWindowTarget,
    root: ContainerVisual,
    armed: Option<Armed>,
    /// 替身正擺在這個視窗正後方等它最小化（見 [`Spare::arm_behind`]）。
    behind: Option<isize>,
}

impl Parts {
    /// 讓縮圖顯示 `hwnd`、擺在 `from`，`shown` 決定看不看得到。狀態沒變
    /// 就不呼叫 DWM。回傳縮圖代號與這次是否真的改了；註冊失敗回傳 `None`。
    fn set(&mut self, hwnd: isize, from: RECT, shown: bool, screen: RECT) -> Option<(isize, bool)> {
        if let Some(armed) = &self.armed
            && armed.hwnd == hwnd
            && armed.from == from
            && armed.shown == shown
        {
            return Some((armed.thumbnail, false));
        }
        let thumbnail = match self.armed.take() {
            Some(armed) if armed.hwnd == hwnd => armed.thumbnail,
            other => {
                if let Some(armed) = other {
                    // SAFETY: Registered here in an earlier call; released once.
                    unsafe {
                        let _ = DwmUnregisterThumbnail(armed.thumbnail);
                    }
                }
                // SAFETY: `stand` is our live window; a closed source fails harmlessly.
                unsafe { DwmRegisterThumbnail(self.stand, HWND(hwnd as *mut _)) }.ok()?
            }
        };
        // The stand-in: the whole window, exactly where it was.
        let properties = DWM_THUMBNAIL_PROPERTIES {
            dwFlags: DWM_TNP_RECTDESTINATION
                | DWM_TNP_VISIBLE
                | DWM_TNP_OPACITY
                | DWM_TNP_SOURCECLIENTAREAONLY,
            rcDestination: RECT {
                left: from.left - screen.left,
                top: from.top - screen.top,
                right: from.right - screen.left,
                bottom: from.bottom - screen.top,
            },
            opacity: 255,
            fVisible: shown.into(),
            fSourceClientAreaOnly: false.into(),
            ..Default::default()
        };
        // SAFETY: A live thumbnail handle.
        unsafe {
            let _ = DwmUpdateThumbnailProperties(thumbnail, &properties);
        }
        self.armed = Some(Armed {
            hwnd,
            thumbnail,
            from,
            shown,
        });
        Some((thumbnail, true))
    }
}

impl Spare {
    /// 蓋住 `screen` 的一組視窗：替身視窗顯示（但沒有內容）、動畫視窗隱藏。
    pub fn new(gfx: &Gfx, screen: RECT) -> Option<Self> {
        let stand = create_window(screen)?;
        let Some((overlay, target, root)) = create_overlay(gfx, screen) else {
            // SAFETY: Our own window, destroyed once.
            unsafe {
                let _ = DestroyWindow(stand);
            }
            return None;
        };
        // SAFETY: Our own live window; it has no content until a thumbnail shows.
        unsafe {
            let _ = ShowWindow(stand, SW_SHOWNOACTIVATE);
        }
        Some(Self {
            screen,
            parts: Some(Parts {
                stand,
                overlay,
                target,
                root,
                armed: None,
                behind: None,
            }),
        })
    }

    /// 是否適用於這個螢幕；螢幕換了（解析度、dock 換螢幕）就得重建。
    #[must_use]
    pub fn covers(&self, screen: RECT) -> bool {
        self.screen == screen
    }

    /// 先替 `hwnd`（前景視窗，最可能被最小化）在 `from`（算法同
    /// `tracker::rect_before_minimize`）註冊好縮圖，先不顯示。
    pub fn watch(&mut self, hwnd: isize, from: RECT) {
        let screen = self.screen;
        if let Some(parts) = self.parts.as_mut()
            && parts.behind.is_none()
        {
            let _ = parts.set(hwnd, from, false, screen);
        }
    }

    /// `hwnd` 馬上要最小化：把它的縮圖顯示在它正後方。視窗本身蓋住縮圖，
    /// 畫面上沒有變化；視窗消失的那一幀，替身就已經在那裡。
    pub fn arm_behind(&mut self, hwnd: isize, from: RECT) {
        let screen = self.screen;
        let Some(parts) = self.parts.as_mut() else {
            return;
        };
        if parts.set(hwnd, from, true, screen).is_none() {
            return;
        }
        // SAFETY: Reorders our own live window; `hwnd` only serves as the
        // place in the z-order, a closed one makes the call fail harmlessly.
        unsafe {
            let _ = SetWindowPos(
                parts.stand,
                Some(HWND(hwnd as *mut _)),
                0,
                0,
                0,
                0,
                SWP_NOMOVE | SWP_NOSIZE | SWP_NOACTIVATE,
            );
            // dock 發起的最小化緊接在後；等這些變更進了合成再讓它開始。
            let _ = DwmFlush();
        }
        parts.behind = Some(hwnd);
    }

    /// 等到的最小化沒有發生（例如按住最小化鈕又移開才放開）：收起縮圖，
    /// 替身視窗回到最上層。
    pub fn disarm(&mut self) {
        let screen = self.screen;
        let Some(parts) = self.parts.as_mut() else {
            return;
        };
        let Some(hwnd) = parts.behind.take() else {
            return;
        };
        if let Some(from) = parts.armed.as_ref().map(|armed| armed.from) {
            let _ = parts.set(hwnd, from, false, screen);
        }
        // SAFETY: Reorders our own live window.
        unsafe {
            let _ = SetWindowPos(
                parts.stand,
                Some(HWND_TOPMOST),
                0,
                0,
                0,
                0,
                SWP_NOMOVE | SWP_NOSIZE | SWP_NOACTIVATE,
            );
        }
    }

    /// 替身正擺在某個視窗後方等它最小化。
    #[must_use]
    pub fn is_armed(&self) -> bool {
        self.parts
            .as_ref()
            .is_some_and(|parts| parts.behind.is_some())
    }
}

impl Drop for Spare {
    fn drop(&mut self) {
        if let Some(parts) = self.parts.take() {
            // SAFETY: Our own handles, released once; a flight that took
            // them left `parts` empty.
            unsafe {
                if let Some(armed) = parts.armed {
                    let _ = DwmUnregisterThumbnail(armed.thumbnail);
                }
                let _ = DestroyWindow(parts.stand);
                let _ = DestroyWindow(parts.overlay);
            }
        }
    }
}

pub struct Flight {
    /// The window being minimized or restored.
    pub hwnd: isize,
    created_at: Instant,
    /// How long the flight lasts (Settings → Genie speed).
    seconds: f32,
    foreground: isize,
    overlay: HWND,
    /// The window under the overlay that holds a minimize's stand-in.
    stand: Option<HWND>,
    /// Kept alive: it owns the overlay's visual tree.
    _target: DesktopWindowTarget,
    root: ContainerVisual,
    /// The monitor the overlay covers.
    screen: RECT,
    /// The frame the genie runs in, and the window's screen rectangle.
    turn: Turn,
    source: RECT,
    genie: Genie,
    /// The dock slot is known (a minimize learns it a little after it starts).
    aimed: bool,
    phase: Phase,
    /// Out of the dock instead of into it.
    restoring: bool,
    /// The real window still has to be restored (restore flights only).
    restore_pending: bool,
    /// Reduced restore pixels, still being prepared on the capture worker.
    captured: Option<(Receiver<IconImage>, RECT)>,
}

unsafe extern "system" fn overlay_proc(hwnd: HWND, msg: u32, w: WPARAM, l: LPARAM) -> LRESULT {
    // SAFETY: Standard forwarding; the overlay handles no messages itself.
    unsafe { DefWindowProcW(hwnd, msg, w, l) }
}

fn intersect(a: RECT, b: RECT) -> RECT {
    RECT {
        left: a.left.max(b.left),
        top: a.top.max(b.top),
        right: a.right.min(b.right),
        bottom: a.bottom.min(b.bottom),
    }
}

fn is_empty(rect: RECT) -> bool {
    rect.right <= rect.left || rect.bottom <= rect.top
}

/// Averages each 2×2 block of pixels into one.
fn halved(image: &IconImage) -> IconImage {
    let (width, height) = ((image.width / 2).max(1), (image.height / 2).max(1));
    let stride = image.width as usize * 4;
    let mut bgra = Vec::with_capacity((width * height * 4) as usize);
    for y in 0..height as usize {
        // The last row or column of a one-pixel-wide image pairs with itself.
        let upper = (y * 2).min(image.height as usize - 1) * stride;
        let lower = (y * 2 + 1).min(image.height as usize - 1) * stride;
        for x in 0..width as usize {
            let left = (x * 2).min(image.width as usize - 1) * 4;
            let right = (x * 2 + 1).min(image.width as usize - 1) * 4;
            for channel in 0..4 {
                let sum = u16::from(image.bgra[upper + left + channel])
                    + u16::from(image.bgra[upper + right + channel])
                    + u16::from(image.bgra[lower + left + channel])
                    + u16::from(image.bgra[lower + right + channel]);
                bgra.push((sum / 4) as u8);
            }
        }
    }
    IconImage {
        width,
        height,
        bgra,
    }
}

/// A transparent, click-through window that covers `screen` during a flight.
fn create_window(screen: RECT) -> Option<HWND> {
    static CLASS: OnceLock<bool> = OnceLock::new();
    let class = w!("HarborFlight");
    // SAFETY: Standard class registration and window creation on the UI
    // thread; the window is destroyed here on failure, or by `Flight::drop`.
    unsafe {
        let instance = GetModuleHandleW(None).ok()?;
        CLASS.get_or_init(|| {
            RegisterClassW(&WNDCLASSW {
                lpfnWndProc: Some(overlay_proc),
                hInstance: instance.into(),
                lpszClassName: class,
                ..Default::default()
            }) != 0
        });
        CreateWindowExW(
            WS_EX_LAYERED
                | WS_EX_TRANSPARENT
                | WS_EX_NOREDIRECTIONBITMAP
                | WS_EX_TOPMOST
                | WS_EX_TOOLWINDOW
                | WS_EX_NOACTIVATE,
            class,
            w!(""),
            WS_POPUP,
            screen.left,
            screen.top,
            screen.right - screen.left,
            screen.bottom - screen.top,
            None,
            None,
            Some(instance.into()),
            None,
        )
        .ok()
    }
}

/// The window the strips are drawn on, with the root of its visual tree.
fn create_overlay(gfx: &Gfx, screen: RECT) -> Option<(HWND, DesktopWindowTarget, ContainerVisual)> {
    let overlay = create_window(screen)?;
    // SAFETY: `overlay` is the live window created above; it is destroyed
    // here on failure, or by `Flight::drop`.
    unsafe {
        let tree = || -> Result<(DesktopWindowTarget, ContainerVisual)> {
            let target = gfx
                .compositor
                .cast::<ICompositorDesktopInterop>()?
                .CreateDesktopWindowTarget(overlay, true)?;
            let root = gfx.compositor.CreateContainerVisual()?;
            root.SetRelativeSizeAdjustment(Vector2 { X: 1.0, Y: 1.0 })?;
            target.SetRoot(&root)?;
            Ok((target, root))
        };
        match tree() {
            Ok((target, root)) => Some((overlay, target, root)),
            Err(_) => {
                let _ = DestroyWindow(overlay);
                None
            }
        }
    }
}

/// Cuts `image` into the strips of a window `window_height` screen pixels tall.
fn cut(
    gfx: &Gfx,
    root: &ContainerVisual,
    image: &IconImage,
    window_height: u32,
) -> Result<Vec<Piece>> {
    let surface = gfx.image_surface(image)?;
    let count = (window_height / STRIP_ROWS).clamp(1, MAX_STRIPS);
    // A full-size image is cut on whole rows, so the first frame matches the
    // screen exactly; a reduced one on fractions, so every strip is stretched
    // by the same amount.
    let exact = image.height == window_height;
    let row = |index: u32| {
        let row = index as f32 * image.height as f32 / count as f32;
        if exact { row.floor() } else { row }
    };
    let compositor = &gfx.compositor;
    let children = root.Children()?;
    (0..count)
        .map(|index| {
            let (top, rows) = (row(index), row(index + 1) - row(index));
            let brush = compositor.CreateSurfaceBrushWithSurface(&surface)?;
            brush.SetStretch(CompositionStretch::None)?;
            brush.SetHorizontalAlignmentRatio(0.0)?;
            brush.SetVerticalAlignmentRatio(0.0)?;
            // Bring this strip's first row to the top of the visual.
            brush.SetTransformMatrix(Matrix3x2::translation(0.0, -top))?;
            // The visual has the strip's natural size; its animated scale
            // squeezes it into place.
            let visual = compositor.CreateSpriteVisual()?;
            visual.SetSize(Vector2 {
                X: image.width as f32,
                Y: rows + OVERLAP_ROWS,
            })?;
            visual.SetBrush(&brush)?;
            children.InsertAtTop(&visual)?;
            Ok(Piece { visual, top, rows })
        })
        .collect()
}

/// Hands the flight to the compositor: every strip follows `genie` for the
/// flight's duration, into the slot or (`reverse`) out of it.
///
/// The expressions are `Genie::frame` written for the compositor: a strip's
/// top is its resting row plus `shift`, its bottom stops at the slot, and its
/// sides follow the cosine funnel scaled by `bend`.
/// Ties every strip to two shared values, `bend` and `shift`, set to the
/// first frame of the flight (the window, or the slot when `reverse`); the
/// strips stay still until [`launch`] animates those values.
fn bind(
    gfx: &Gfx,
    genie: &Genie,
    pieces: &[Piece],
    image: (f32, f32),
    reverse: bool,
) -> Result<CompositionPropertySet> {
    let compositor = &gfx.compositor;
    let (window, slot) = (genie.window, genie.slot);
    let travel = (slot.bottom - window.top).max(0.0);
    let at = |progress: f32| if reverse { 1.0 - progress } else { progress };

    let drive = compositor.CreatePropertySet()?;
    drive.InsertScalar(h!("bend"), Genie::bend(at(0.0)))?;
    drive.InsertScalar(h!("shift"), Genie::slide(at(0.0)) * travel)?;

    // How far along the funnel the middle of the strip is, eased by a cosine.
    let curve = format!(
        "((1 - Cos(3.14159265 * Clamp(((y0 + Min(y1, {bottom:.3} - d.shift)) * 0.5 + d.shift - ({top:.3})) / {span:.3}, 0, 1))) * 0.5 * d.bend)",
        bottom = slot.bottom,
        top = window.top,
        span = (slot.top - window.top).max(1.0),
    );
    let offset = format!(
        "Vector3(({left:.3}) + ({shrink:.3}) * {curve}, y0 + d.shift, 0)",
        left = window.left,
        shrink = slot.left - window.left,
    );
    let scale = format!(
        "Vector3((({width:.3}) + ({narrow:.3}) * {curve}) / {source:.3}, Max(Min(y1, {bottom:.3} - d.shift) - y0, 0) / rows, 1)",
        width = window.right - window.left,
        narrow = (slot.right - slot.left) - (window.right - window.left),
        source = image.0,
        bottom = slot.bottom,
    );
    let offset = compositor.CreateExpressionAnimationWithExpression(&HSTRING::from(offset))?;
    let scale = compositor.CreateExpressionAnimationWithExpression(&HSTRING::from(scale))?;
    offset.SetReferenceParameter(h!("d"), &drive)?;
    scale.SetReferenceParameter(h!("d"), &drive)?;

    // Screen rows per source row.
    let stretch = (window.bottom - window.top) / image.1;
    for piece in pieces {
        let y0 = window.top + piece.top * stretch;
        let y1 = y0 + piece.rows * stretch;
        // Each start takes a copy of the animation with these values.
        offset.SetScalarParameter(h!("y0"), y0)?;
        offset.SetScalarParameter(h!("y1"), y1)?;
        scale.SetScalarParameter(h!("y0"), y0)?;
        scale.SetScalarParameter(h!("y1"), y1)?;
        scale.SetScalarParameter(h!("rows"), piece.rows)?;
        piece.visual.StartAnimation(h!("Offset"), &offset)?;
        piece.visual.StartAnimation(h!("Scale"), &scale)?;
    }
    Ok(drive)
}

/// Starts the flight bound by [`bind`]: the compositor animates `bend` and
/// `shift` for `seconds` at the display's refresh rate.
///
/// 回傳的旗標在合成器真的播完時才會變 `true`。UI 執行緒自己的時鐘在負載高時
/// 會和合成器脫鉤（動畫提交晚於計時起點），只看時鐘收尾，會在條帶還在半空
/// 時就拆掉覆蓋層、或提早還原真視窗，畫面就「閃一下」。
fn launch(
    gfx: &Gfx,
    genie: &Genie,
    drive: &CompositionPropertySet,
    reverse: bool,
    seconds: f32,
) -> Result<Arc<AtomicBool>> {
    let compositor = &gfx.compositor;
    let travel = (genie.slot.bottom - genie.window.top).max(0.0);
    let at = |progress: f32| if reverse { 1.0 - progress } else { progress };
    let linear = compositor.CreateLinearEasingFunction()?;
    let duration = TimeSpan {
        Duration: (seconds * 1e7) as i64,
    };
    let animate = |name: &HSTRING, curve: fn(f32) -> f32, amount: f32| -> Result<()> {
        let animation = compositor.CreateScalarKeyFrameAnimation()?;
        animation.SetDuration(duration)?;
        for step in 0..=KEYFRAMES {
            let progress = step as f32 / KEYFRAMES as f32;
            animation.InsertKeyFrameWithEasingFunction(
                progress,
                curve(at(progress)) * amount,
                &linear,
            )?;
        }
        drive.StartAnimation(name, &animation)?;
        Ok(())
    };
    // 批次只包住這兩段關鍵影格動畫（條帶上的運算式動畫是在 `bind` 啟動的，
    // 不會讓批次永遠無法結束）。
    let done = Arc::new(AtomicBool::new(false));
    let batch = compositor.CreateScopedBatch(CompositionBatchTypes::Animation)?;
    let finished = Arc::clone(&done);
    batch.Completed(&TypedEventHandler::new(move |_, _| {
        finished.store(true, Ordering::Release);
        Ok(())
    }))?;
    animate(h!("bend"), Genie::bend, 1.0)?;
    animate(h!("shift"), Genie::slide, travel)?;
    batch.End()?;
    Ok(done)
}

/// Nothing is drawn below the slot: strips vanish as they enter it.
fn clip_at_slot(gfx: &Gfx, root: &ContainerVisual, genie: &Genie, turn: Turn) -> Result<()> {
    let below = turn.size().1 - genie.slot.bottom;
    let clip = gfx
        .compositor
        .CreateInsetClipWithInsets(0.0, 0.0, 0.0, below.max(0.0))?;
    root.SetClip(&clip)?;
    Ok(())
}

fn local(rect: RECT, screen: RECT) -> Edges {
    Edges {
        left: (rect.left - screen.left) as f32,
        top: (rect.top - screen.top) as f32,
        right: (rect.right - screen.left) as f32,
        bottom: (rect.bottom - screen.top) as f32,
    }
}

impl Flight {
    /// Puts up the stand-in of a window that is being minimized, where it was
    /// (`from`, a screen rectangle), on the windows of `spare`, which cover
    /// the monitor. The flight takes off once [`Flight::aim`] gives it its slot.
    ///
    /// `from` is the whole window, invisible borders included, where its live
    /// thumbnail goes; `frame` is the part that showed on screen, which the
    /// genie copies and flies (the work area for a maximized window, so no
    /// strip of the menu bar or desktop around it comes along).
    pub fn start(
        mut spare: Spare,
        hwnd: isize,
        from: RECT,
        frame: RECT,
        seconds: f32,
        edge: DockEdge,
    ) -> Option<Self> {
        let screen = spare.screen;
        let turn = Turn::new(edge, screen);
        // A timed-out GDI call cannot be cancelled safely. Keep at most one
        // outstanding copy, including a worker whose overlay was abandoned.
        let capture_permit = CapturePermit::acquire()?;
        // Only what is on screen can be copied from it.
        let visible = intersect(frame, screen);
        if is_empty(visible) {
            return None;
        }
        // The stand-in has a window of its own, under the strips: DWM draws a
        // thumbnail over everything else in its window, and there it would
        // hide the first frames of the flight. Normally it is already up
        // (see `Spare`); on any early return below, dropping `spare` cleans up.
        let parts = spare.parts.as_mut()?;
        let (thumbnail, changed) = parts.set(hwnd, from, true, screen)?;
        // SAFETY: Our own live windows.
        unsafe {
            if changed {
                // The window has already vanished. Without the flush, DWM
                // applied the new thumbnail properties a frame late much more
                // often (measured about 1 in 3, against 1 in 12 with it).
                let _ = DwmFlush();
            }
            // The overlay goes on top only now; the stand-in then goes right
            // under it, above any other topmost window.
            let _ = ShowWindow(parts.overlay, SW_SHOWNOACTIVATE);
            let _ = SetWindowPos(
                parts.stand,
                Some(parts.overlay),
                0,
                0,
                0,
                0,
                SWP_NOMOVE | SWP_NOSIZE | SWP_NOACTIVATE | SWP_SHOWWINDOW,
            );
        }
        turn.apply(&parts.root).ok()?;
        // Copying the screen takes 30-80 ms however small the rectangle, so it
        // runs on a worker from the first moment instead of holding the dock.
        // GDI reads what DWM composes next, and that includes the stand-in.
        let (sender, shot) = channel();
        thread::Builder::new()
            .name("harbor-capture".into())
            .spawn(move || {
                let captured = screen::capture(visible);
                // 擷取完就放行下一次擷取，縮小還原用快照不必佔著名額。
                drop(capture_permit);
                let Some(image) = captured else {
                    let _ = sender.send(None);
                    return;
                };
                let image = turn.image(image);
                // 動畫只等擷取本身：縮小快照（debug build 要 100-600 ms，
                // 視窗越大越久）放到送出之後，否則神燈起飛前會停頓一下。
                let (restore_sender, restore) = channel();
                let _ = sender.send(Some(Captured {
                    image: image.clone(),
                    restore,
                }));
                let _ = restore_sender.send(halved(&image));
            })
            .ok()?;
        // From here on the flight owns the windows and the thumbnail.
        let Parts {
            stand,
            overlay,
            target,
            root,
            ..
        } = spare.parts.take()?;
        let window = turn.edges(local(visible, screen));
        Some(Self {
            hwnd,
            created_at: Instant::now(),
            seconds,
            foreground: tracker::foreground_window(),
            overlay,
            stand: Some(stand),
            _target: target,
            root,
            screen,
            turn,
            source: visible,
            // Until `aim`, the slot is the window itself.
            genie: Genie {
                window,
                slot: window,
            },
            aimed: false,
            phase: Phase::Standing {
                thumbnail,
                frames: 0,
                shot,
                started: Instant::now(),
            },
            restoring: false,
            restore_pending: false,
            captured: None,
        })
    }

    /// Flies a minimized window out of its dock slot (`slot`, a screen
    /// rectangle) back to where `snapshot` was taken. The caller restores the
    /// real window when [`Flight::take_restore`] says so.
    pub fn restore(
        gfx: &Gfx,
        hwnd: isize,
        snapshot: &Snapshot,
        slot: RECT,
        screen: RECT,
        seconds: f32,
        edge: DockEdge,
    ) -> Option<Self> {
        // A copy turned for another edge (the dock moved meanwhile) would
        // fly sideways: the window just comes back.
        let source = intersect(snapshot.window, screen);
        if is_empty(source) || snapshot.edge != edge {
            return None;
        }
        let turn = Turn::new(edge, screen);
        let (overlay, target, root) = create_overlay(gfx, screen)?;
        let genie = Genie {
            window: turn.edges(local(source, screen)),
            slot: turn.edges(local(slot, screen)),
        };
        let height = (genie.window.bottom - genie.window.top) as u32;
        let image = &snapshot.image;
        let size = (image.width as f32, image.height as f32);
        let drive = turn
            .apply(&root)
            .and_then(|()| clip_at_slot(gfx, &root, &genie, turn))
            .and_then(|()| cut(gfx, &root, image, height))
            .and_then(|pieces| bind(gfx, &genie, &pieces, size, true))
            .and_then(|drive| {
                launch(gfx, &genie, &drive, true, seconds).map(|done| (drive, done))
            });
        let Ok((drive, done)) = drive else {
            // SAFETY: Our own window, destroyed once.
            unsafe {
                let _ = DestroyWindow(overlay);
            }
            return None;
        };
        let flight = Self {
            hwnd,
            created_at: Instant::now(),
            seconds,
            foreground: tracker::foreground_window(),
            overlay,
            stand: None,
            _target: target,
            root,
            screen,
            turn,
            source,
            genie,
            aimed: true,
            phase: Phase::Flying {
                start: Instant::now(),
                _drive: drive,
                done,
                stand_in: None,
            },
            restoring: true,
            restore_pending: true,
            captured: None,
        };
        // SAFETY: Our own live window.
        unsafe {
            let _ = ShowWindow(flight.overlay, SW_SHOWNOACTIVATE);
        }
        Some(flight)
    }

    /// Advances the animation; returns `false` once it has finished (or cannot go on).
    pub fn step(&mut self, gfx: &Gfx, now: Instant) -> bool {
        let elapsed = now.saturating_duration_since(self.created_at);
        if elapsed >= FLIGHT_TIMEOUT
            || (matches!(self.phase, Phase::Standing { .. }) && elapsed >= CAPTURE_TIMEOUT)
        {
            self.restore_pending = false;
            return false;
        }
        match &mut self.phase {
            Phase::Standing {
                thumbnail,
                frames,
                shot,
                started,
            } => {
                if now.saturating_duration_since(*started) >= CAPTURE_TIMEOUT {
                    return false;
                }
                *frames += 1;
                // No slot to fly to: the window just stays minimized.
                if !self.aimed {
                    return *frames <= FRAMES_TO_AIM;
                }
                let image = match shot.try_recv() {
                    Ok(Some(image)) => image,
                    Err(TryRecvError::Empty) => return true,
                    Ok(None) | Err(TryRecvError::Disconnected) => return false,
                };
                let thumbnail = *thumbnail;
                let Ok((drive, done)) = self.take_off(gfx, image) else {
                    return false;
                };
                // The clock starts now, so copying the screen does not eat
                // the animation.
                self.phase = Phase::Flying {
                    start: Instant::now(),
                    _drive: drive,
                    done,
                    stand_in: Some((thumbnail, 0)),
                };
                true
            }
            // The compositor is flying the strips; only the end is ours.
            Phase::Flying {
                start,
                stand_in,
                done,
                ..
            } => {
                if let Some((thumbnail, frames)) = stand_in {
                    *frames += 1;
                    if *frames > HANDOVER_FRAMES {
                        // The strips are on screen by now: the stand-in can go.
                        // SAFETY: `thumbnail` came from DwmRegisterThumbnail and is released once.
                        unsafe {
                            let _ = DwmUnregisterThumbnail(*thumbnail);
                        }
                        *stand_in = None;
                    }
                }
                let elapsed = now.saturating_duration_since(*start);
                let t = elapsed.as_secs_f32() / self.seconds;
                // 時鐘到了還要等合成器真的播完：否則動畫尾段被提早腰斬，
                // 還原時真視窗也會在條帶還沒到全尺寸前就冒出來。
                let finished = done.load(Ordering::Acquire)
                    || elapsed >= Duration::from_secs_f32(self.seconds) + COMPLETION_GRACE;
                if t < 1.0 || !finished {
                    return true;
                }
                if !self.restoring {
                    return false;
                }
                self.phase = Phase::Settling {
                    until: now + SETTLE,
                };
                true
            }
            Phase::Settling { until } => {
                // 真正的視窗是非同步還原的（ShowWindowAsync），慢的 app 超過
                // SETTLE 還沒出現；此時收掉覆蓋層，使用者會先看到空白／舊畫面
                // 再閃出視窗。所以視窗還在最小化就一直延後，直到它出現後
                // 再多撐一個 SETTLE 讓它畫好第一幀（上限由 FLIGHT_TIMEOUT 保底）。
                // SAFETY: Read-only query on a window handle; a dead handle returns false.
                if unsafe { IsIconic(HWND(self.hwnd as *mut _)) }.as_bool() {
                    *until = now + SETTLE;
                }
                now < *until
            }
        }
    }

    /// Gives a minimize its dock slot (a screen rectangle).
    pub fn aim(&mut self, gfx: &Gfx, slot: RECT) {
        self.genie.slot = self.turn.edges(local(slot, self.screen));
        self.aimed = true;
        // Without the clip the flight still works; strips would only show a
        // little below the slot.
        let _ = clip_at_slot(gfx, &self.root, &self.genie, self.turn);
    }

    /// Puts the flight right under `dock` in the z-order, so a window flying
    /// into (or out of) an app icon passes behind the dock's glass instead
    /// of over it. The stand-in window stays just under the strips.
    pub fn tuck_under(&self, dock: HWND) {
        let flags = SWP_NOMOVE | SWP_NOSIZE | SWP_NOACTIVATE;
        // SAFETY: Reorders our own live windows; nothing else changes.
        unsafe {
            let _ = SetWindowPos(self.overlay, Some(dock), 0, 0, 0, 0, flags);
            if let Some(stand) = self.stand {
                let _ = SetWindowPos(stand, Some(self.overlay), 0, 0, 0, 0, flags);
            }
        }
    }

    /// 這趟是從 dock 飛出來還原。
    #[must_use]
    pub fn is_restoring(&self) -> bool {
        self.restoring
    }

    #[must_use]
    pub fn needs_aim(&self) -> bool {
        !self.aimed
    }

    /// A delayed restore belongs to the original activation request. Once
    /// another app wins the foreground, the overlay and request are stale.
    pub fn is_current(&self, foreground: isize) -> bool {
        activation_is_current(self.restoring, self.foreground, self.hwnd, foreground)
    }

    /// Width of the flying window divided by its height, on screen.
    #[must_use]
    pub fn aspect(&self) -> f32 {
        let window = self.source;
        (window.right - window.left) as f32 / (window.bottom - window.top).max(1) as f32
    }

    /// `true`, once, when the real window has to be restored: the restore
    /// animation has reached full size, or has ended early.
    pub fn take_restore(&mut self, finished: bool) -> bool {
        let due = finished || matches!(self.phase, Phase::Settling { .. });
        due && std::mem::take(&mut self.restore_pending)
    }

    /// The snapshot of a window that has just been minimized, to restore it
    /// with the genie later.
    pub fn take_snapshot_for_restore(&mut self) -> Option<Snapshot> {
        let edge = self.turn.edge;
        let (restore, window) = self.captured.take()?;
        // 飛行結束時縮小早就做完；極少數還沒好（很大的視窗）就稍等一下，
        // 最多 100 ms，等不到就放棄這張快照（下次還原沒有神燈，但不卡住）。
        let image = restore.recv_timeout(Duration::from_millis(100)).ok()?;
        Some(Snapshot {
            image,
            window,
            edge,
        })
    }

    /// Opacity (0..=255) of the dock's own thumbnail for this window: it
    /// fades in as the genie lands, and out as it takes off.
    pub fn landing_opacity(&self, now: Instant) -> u8 {
        let Phase::Flying { start, .. } = &self.phase else {
            return 0;
        };
        let elapsed = now.saturating_duration_since(*start).as_secs_f32() / self.seconds;
        let t = if self.restoring {
            1.0 - elapsed
        } else {
            elapsed
        };
        (((t - LANDING_START) / (1.0 - LANDING_START)).clamp(0.0, 1.0) * 255.0) as u8
    }

    /// Cuts the copy of the window (`image`) into strips, ties them to the
    /// genie and starts it.
    fn take_off(
        &mut self,
        gfx: &Gfx,
        captured: Captured,
    ) -> Result<(CompositionPropertySet, Arc<AtomicBool>)> {
        let Captured { image, restore } = captured;
        let window = self.source;
        let pieces = cut(gfx, &self.root, &image, image.height)?;
        let size = (image.width as f32, image.height as f32);
        let drive = bind(gfx, &self.genie, &pieces, size, false)?;
        let done = launch(gfx, &self.genie, &drive, false, self.seconds)?;
        self.captured = Some((restore, window));
        Ok((drive, done))
    }
}

fn activation_is_current(
    restoring: bool,
    requested_from: isize,
    target: isize,
    foreground: isize,
) -> bool {
    // Minimization normally hands focus to the next app. Only a delayed
    // restoration could steal focus back and must be cancelled on that switch.
    !restoring || foreground == 0 || foreground == requested_from || foreground == target
}

impl Drop for Flight {
    fn drop(&mut self) {
        // SAFETY: The handles were created in `start` or `restore` and are
        // released once.
        unsafe {
            if let Phase::Standing { thumbnail, .. }
            | Phase::Flying {
                stand_in: Some((thumbnail, _)),
                ..
            } = &self.phase
            {
                let thumbnail = *thumbnail;
                let _ = DwmUnregisterThumbnail(thumbnail);
            }
            if let Some(stand) = self.stand {
                let _ = DestroyWindow(stand);
            }
            let _ = DestroyWindow(self.overlay);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn edges(left: f32, top: f32, right: f32, bottom: f32) -> Edges {
        Edges {
            left,
            top,
            right,
            bottom,
        }
    }

    /// The frame shown through `matrix` puts every point back where it was
    /// on screen, and the slot (at the dock) is "below" the window in it.
    #[test]
    fn a_turned_frame_maps_back_to_the_screen_and_flows_toward_the_dock() {
        let screen = RECT {
            left: 0,
            top: 0,
            right: 1920,
            bottom: 1080,
        };
        let window = edges(600.0, 200.0, 1400.0, 800.0);
        for (edge, slot) in [
            (DockEdge::Left, edges(20.0, 500.0, 70.0, 550.0)),
            (DockEdge::Right, edges(1850.0, 500.0, 1900.0, 550.0)),
        ] {
            let turn = Turn::new(edge, screen);
            let m = turn.matrix().unwrap_or_default();
            let show =
                |x: f32, y: f32| (x * m.M11 + y * m.M21 + m.M41, x * m.M12 + y * m.M22 + m.M42);
            let frame = turn.edges(window);
            // Two opposite corners land on two opposite corners of the window.
            let corners = [show(frame.left, frame.top), show(frame.right, frame.bottom)];
            for (x, y) in corners {
                assert!(x == window.left || x == window.right, "{edge:?} {x}");
                assert!(y == window.top || y == window.bottom, "{edge:?} {y}");
            }
            let slot = turn.edges(slot);
            assert!(
                slot.top > frame.bottom,
                "{edge:?}: the slot is beyond the window"
            );
            let (width, height) = turn.size();
            assert!(frame.right <= width && slot.bottom <= height);
        }
    }

    #[test]
    fn a_turned_copy_keeps_every_pixel_once() {
        // 3x2 image, each pixel's blue channel its index.
        let image = IconImage {
            width: 3,
            height: 2,
            bgra: (0..6u8).flat_map(|i| [i, 0, 0, 255]).collect(),
        };
        let screen = RECT {
            left: 0,
            top: 0,
            right: 1920,
            bottom: 1080,
        };
        for edge in [DockEdge::Left, DockEdge::Right] {
            let turned = Turn::new(edge, screen).image(image.clone());
            assert_eq!((turned.width, turned.height), (2, 3));
            let mut seen: Vec<u8> = turned.bgra.chunks(4).map(|px| px[0]).collect();
            seen.sort_unstable();
            assert_eq!(seen, vec![0, 1, 2, 3, 4, 5]);
        }
        let same = Turn::new(DockEdge::Bottom, screen).image(image.clone());
        assert_eq!(same, image);
    }

    #[test]
    fn minimizing_keeps_its_capture_when_focus_passes_to_the_next_window() {
        assert!(activation_is_current(false, 10, 10, 20));
    }

    #[test]
    fn restoring_cannot_take_focus_back_after_the_user_switches_apps() {
        assert!(!activation_is_current(true, 10, 30, 20));
        assert!(activation_is_current(true, 10, 30, 10));
        assert!(activation_is_current(true, 10, 30, 30));
        assert!(activation_is_current(true, 10, 30, 0));
    }

    #[test]
    fn restore_pixels_average_each_block_and_keep_alpha() {
        let image = IconImage {
            width: 2,
            height: 2,
            bgra: vec![
                0, 10, 20, 255, 20, 30, 40, 255, 40, 50, 60, 255, 60, 70, 80, 255,
            ],
        };
        let reduced = halved(&image);
        assert_eq!((reduced.width, reduced.height), (1, 1));
        assert_eq!(reduced.bgra, [30, 40, 50, 255]);
    }

    #[test]
    fn restore_pixels_accept_a_single_pixel_window() {
        let image = IconImage {
            width: 1,
            height: 1,
            bgra: vec![1, 2, 3, 255],
        };
        assert_eq!(halved(&image), image);
    }

    #[test]
    fn stalled_capture_keeps_its_slot_until_the_worker_finishes() {
        let first = CapturePermit::acquire();
        assert!(first.is_some());
        assert!(CapturePermit::acquire().is_none());
        drop(first);
        assert!(CapturePermit::acquire().is_some());
    }
}

#[cfg(test)]
mod native_tests {
    use super::*;
    use windows::Win32::{
        System::WinRT::{
            CreateDispatcherQueueController, DQTAT_COM_STA, DQTYPE_THREAD_CURRENT,
            DispatcherQueueOptions,
        },
        UI::WindowsAndMessaging::{IsWindow, WS_OVERLAPPEDWINDOW},
    };

    #[test]
    #[ignore = "requires an interactive Windows desktop and DWM"]
    fn native_stalled_capture_should_release_overlays_without_late_restore() -> Result<()> {
        struct Source(HWND);
        impl Drop for Source {
            fn drop(&mut self) {
                unsafe {
                    let _ = DestroyWindow(self.0);
                }
            }
        }
        let _queue = unsafe {
            CreateDispatcherQueueController(DispatcherQueueOptions {
                dwSize: size_of::<DispatcherQueueOptions>() as u32,
                threadType: DQTYPE_THREAD_CURRENT,
                apartmentType: DQTAT_COM_STA,
            })?
        };
        let source = Source(unsafe {
            CreateWindowExW(
                WS_EX_TOOLWINDOW | WS_EX_NOACTIVATE,
                w!("STATIC"),
                w!("Harbor animation test"),
                WS_OVERLAPPEDWINDOW,
                10,
                10,
                240,
                160,
                None,
                None,
                None,
                None,
            )?
        });
        unsafe {
            let _ = ShowWindow(source.0, SW_SHOWNOACTIVATE);
        }
        let gfx = Gfx::new()?;
        let screen = RECT {
            left: 0,
            top: 0,
            right: 640,
            bottom: 480,
        };
        let from = RECT {
            left: 10,
            top: 10,
            right: 250,
            bottom: 170,
        };
        let spare = Spare::new(&gfx, screen)
            .ok_or_else(|| anyhow::anyhow!("could not create the stand-in windows"))?;
        let mut flight = Flight::start(
            spare,
            source.0.0 as isize,
            from,
            from,
            0.5,
            DockEdge::Bottom,
        )
        .ok_or_else(|| anyhow::anyhow!("could not create the native animation"))?;
        let overlay = flight.overlay;
        let stand = flight.stand;
        let unrelated = if flight.foreground != 123 { 123 } else { 124 };
        assert!(
            !flight.is_current(unrelated),
            "switching apps invalidates the animation"
        );
        // Replace the graphics worker with one that never delivers a result.
        let (_sender, stalled) = channel();
        if let Phase::Standing { shot, .. } = &mut flight.phase {
            *shot = stalled;
        }
        flight.aim(
            &gfx,
            RECT {
                left: 300,
                top: 400,
                right: 340,
                bottom: 440,
            },
        );
        flight.created_at = Instant::now() - CAPTURE_TIMEOUT;
        flight.restore_pending = true;
        assert!(!flight.step(&gfx, Instant::now()));
        assert!(
            !flight.take_restore(true),
            "timing out must not reactivate a stale target"
        );
        drop(flight);
        assert!(!unsafe { IsWindow(Some(overlay)) }.as_bool());
        if let Some(stand) = stand {
            assert!(!unsafe { IsWindow(Some(stand)) }.as_bool());
        }
        Ok(())
    }
}
