use anyhow::*;
use winit::window;
use winit::event_loop::{EventLoop, EventLoopWindowTarget, ControlFlow};
use winit::platform::windows::{EventLoopExtWindows, WindowBuilderExtWindows, WindowExtWindows};
use winapi::shared::windef::HWND;
use winapi::um::winuser::*;
use winapi::um::dwmapi::DwmExtendFrameIntoClientArea;
use winit::event::{Event, WindowEvent};
use winapi::um::uxtheme::MARGINS;
use imgui_winit_support::{WinitPlatform, HiDpiMode};
use std::time::Instant;
use imgui::Ui;
use glium::Display;
use winapi::um::processthreadsapi::GetCurrentProcessId;
use crate::util::set_remote_affinity;
use winutil::get_windows;

const HIJACK_WINDOWS: &[(&str, &str)] = &[
    ("CEF-OSC-WIDGET", "NVIDIA GeForce Overlay")
];

pub struct OverlayWindow {
    pub event_loop: EventLoop<()>,
    pub display: glium::Display,
    pub controller: WindowController,
}

unsafe impl Send for OverlayWindow {}

impl OverlayWindow {
    /// Hijacks a Window or creates one if there are no windows available to hijack
    pub fn new() -> Result<Self> {
        match Self::find_hijack() {
            Ok(n) => Ok(n),
            Err(e) => {
                Self::create()
            }
        }
    }

    /// Finds a secure overlay window that it can hijack (nvidia for example)
    pub fn find_hijack() -> Result<Self> {
        for window in HIJACK_WINDOWS {
            match Self::hijack(window.0, window.1) {
                Some(Ok(window)) => return Ok(window),
                Some(Err(e)) => return Err(e),
                _ => {}
            }
        }
        bail!("Could not find window to hijack")
    }

    pub fn hijack(class_name: &str, window_name: &str) -> Option<Result<Self>> {
        get_windows().into_iter()
            .find(|window| {
                window.class == class_name && window.title.as_ref().unwrap() == window_name
            })
            .map(|window| Self::from_hwnd(window.hwnd))
    }

    /// Creates a new window to use as an overlay
    pub fn create() -> Result<Self> {
        Self::from_window_builder(winit::window::WindowBuilder::new())
    }

    /// Creates a window by hijacking another HWND
    fn from_hwnd(hwnd: HWND) -> Result<Self> {
        Self::from_window_builder(winit::window::WindowBuilder::new()
            .with_custom_hwnd(hwnd)
            .with_drag_and_drop(false))
    }

    fn from_window_builder(window_builder: winit::window::WindowBuilder) -> Result<Self> {
        let window_builder = window_builder.with_hwnd_callback(Self::init_overlay);

        let event_loop = EventLoop::new_any_thread();

        let builder = glutin::ContextBuilder::new();
        let windowed_context = builder.build_windowed(window_builder, &event_loop)?;
        let hwnd = windowed_context.window().hwnd() as HWND;
        let display = Display::from_gl_window(windowed_context)?;

        Ok(Self { event_loop, display, controller: WindowController::new(hwnd, None) })
    }

    /// Modifies the HWND to be an overlay
    unsafe fn init_overlay(hwnd: HWND) {
        // SetWindowLongA(hwnd, GWL_STYLE, (WS_CLIPSIBLINGS | WS_POPUP | WS_VISIBLE) as _);
        SetWindowLongA(hwnd, GWL_STYLE, (WS_CLIPSIBLINGS | WS_POPUP | WS_VISIBLE) as _);
        // Default Nvidia flags: EX_LAYERED, EX_NOACTIVE, EX_TOOLWINDOW
        SetWindowLongA(hwnd, GWL_EXSTYLE, (WS_EX_TRANSPARENT | WS_EX_LAYERED | WS_EX_TOOLWINDOW) as _);
        // SetWindowLongA(hwnd, GWL_EXSTYLE, (WS_EX_ACCEPTFILES | WS_EX_APPWINDOW | WS_EX_TRANSPARENT | WS_EX_WINDOWEDGE) as _);

        // Remove border
        DwmExtendFrameIntoClientArea(hwnd, &MARGINS {
            cxLeftWidth: -1,
            cxRightWidth: -1,
            cyBottomHeight: -1,
            cyTopHeight: -1,
        });

        // Make transparent
        SetLayeredWindowAttributes(hwnd, 0, 0xFF, 0x02);

        ShowWindow(hwnd, SW_SHOW);
        UpdateWindow(hwnd);
    }
}

/// A struct that applies necessary updates to an overlay window
pub struct WindowController {
    pub hwnd: HWND,
    pub target_hwnd: Option<HWND>,
    last_update: Instant,
    last_clickthrough: bool,
    last_foreground_window: Option<HWND>,
}

impl WindowController {
    pub fn new(hwnd: HWND, target_hwnd: Option<HWND>) -> Self {
        Self { hwnd, target_hwnd, last_update: Instant::now(), last_clickthrough: false, last_foreground_window: None }
    }

    pub(crate) fn update(&mut self) {
        unsafe {
            // Update overlay location to be on top of target
            if let Some(target) = self.target_hwnd {
                let mut rect = std::mem::zeroed();
                GetWindowRect(target, &mut rect);

                if rect.bottom == 0 && rect.top == 0 {
                    panic!("Error updating target window: could not get the window rect");
                }

                let width = rect.right - rect.left;
                let height = rect.bottom - rect.top;

                // FIXME: For some reason if this is set to any full screen app, it thinks
                // that its in fullscreen windowed mode and the background turns black. For
                // now making the window slightly smaller seems to fix the issue
                MoveWindow(self.hwnd, rect.left + 1, rect.top + 1, width - 1, height - 1, 1);
            }

            let target = self.target_hwnd.unwrap_or_else(|| GetForegroundWindow());
            let target = GetWindow(target, GW_HWNDPREV);
            if target != self.hwnd {
                SetWindowPos(
                    self.hwnd,
                    target,
                    0, 0, 0, 0,
                    SWP_ASYNCWINDOWPOS | SWP_NOMOVE | SWP_NOSIZE,
                );
                UpdateWindow(self.hwnd);
            }

            if !self.last_clickthrough && GetForegroundWindow() == target {
                SetForegroundWindow(self.hwnd);
            }
        }
    }

    pub fn set_target(&mut self, target: Option<HWND>) {
        self.target_hwnd = target;
    }

    /// Enables or disables window clickthrough
    pub fn clickthrough(&mut self, clickthrough: bool) {
        if clickthrough != self.last_clickthrough {
            self.last_clickthrough = clickthrough;

            unsafe {
                self.set_style_flag(GWL_EXSTYLE, WS_EX_TRANSPARENT, clickthrough);

                /*
                match clickthrough {
                    false => {
                        self.last_foreground_window = Some(GetForegroundWindow());
                        println!("setforeground");
                        SetForegroundWindow(self.hwnd);
                    }
                    true => {
                        if let Some(window) = self.last_foreground_window {
                            SetForegroundWindow(window);
                            self.last_foreground_window = None;
                        }
                    }
                }*/
            };
        }
    }

    /// Enables or disables hiding from screenshots
    pub fn hide_screenshots(&mut self, hide: bool) {
        unsafe { self.set_affinity(if hide { WindowAffinity::WdaExcludeFromCapture } else { WindowAffinity::WdaNone }).expect("Could not set affinity") }
    }

    unsafe fn set_affinity(&self, affinity: WindowAffinity) -> Result<()> {
        if self.get_affinity() == affinity {
            return Ok(());
        }
        unsafe {
            // If the HWND is owned by this process, we can just call swda
            if GetCurrentProcessId() == self.get_owner_pid() {
                let result = SetWindowDisplayAffinity(self.hwnd, affinity as _);
                if result == 0 {
                    bail!("SetWindowDisplayAffinity failed: {}", std::io::Error::last_os_error())
                } else {
                    Ok(())
                }
            } else { // otherwise, we have to set it remotely
                set_remote_affinity(self.hwnd, affinity);
                let actual_affinity = self.get_affinity();
                if affinity != actual_affinity {
                    bail!("Setting remote affinity did not work. affinity: {:?}, actual_affinity: {:?}", affinity, actual_affinity);
                }
                Ok(())
            }
        }
    }

    unsafe fn get_owner_pid(&self) -> u32 {
        let mut pid = 0;
        GetWindowThreadProcessId(self.hwnd, &mut pid);
        if pid == 0 {
            panic!("GetWindowThreadProcessId failed");
        }
        pid
    }

    unsafe fn get_affinity(&self) -> WindowAffinity {
        let mut affinity = WindowAffinity::WdaNone;
        GetWindowDisplayAffinity(self.hwnd, std::mem::transmute(&mut affinity));
        affinity
    }

    unsafe fn set_style_flag(&self, n_index: i32, flag: u32, enabled: bool) {
        let style = GetWindowLongA(self.hwnd, n_index) as u32;
        let style = match enabled {
            true => style | flag,
            false => style & !flag,
        };
        SetWindowLongA(self.hwnd, n_index, style as _);
    }
}

// https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-setwindowdisplayaffinity
#[repr(u32)]
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum WindowAffinity {
    /// Imposes no restrictions on where the window can be displayed.
    WdaNone = 0x0,

    /// The window content is displayed only on a monitor. Everywhere else, the window appears with no content.
    WdaMonitor = 0x1,

    /// The window is displayed only on a monitor. Everywhere else, the window does not appear at all.
    /// One use for this affinity is for windows that show video recording controls, so that the controls are not included in the capture.
    /// Introduced in Windows 10 Version 2004. See remarks about compatibility regarding previous versions of Windows.
    WdaExcludeFromCapture = 0x11,
}