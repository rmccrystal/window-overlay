use winapi::um::winuser;
use winapi::shared::windef::HWND;
use std::ffi::c_void;
use std::mem;
use winapi::shared::minwindef::{LPARAM, BOOL};
use winapi::um::winuser::{EnumWindows, GetWindowThreadProcessId};
use log::*;
use winapi::um::libloaderapi::{LoadLibraryA, GetProcAddress, FreeLibrary};
use winutil::inject_func;
use crate::window::WindowAffinity;
use anyhow::*;

#[macro_export]
macro_rules! win_str {
    ($string: expr) => {
        format!("{}{}", $string, "\0").as_ptr() as *const i8
    };
}

/// Sets the window affinity when the HWND isn't owned by this process (nvidia for example)
pub(crate) unsafe fn set_remote_affinity(hwnd: HWND, affinity: WindowAffinity) -> Result<()> {
    debug!("Remotely setting affinity to {:?}", affinity);

    let pid = {
        let mut pid = 0;
        GetWindowThreadProcessId(hwnd, &mut pid);
        if pid == 0 {
            panic!("GetWindowThreadProcessId failed");
        }
        pid
    };

    let user32 = LoadLibraryA(win_str!("user32.dll"));
    let swda = GetProcAddress(
        user32,
        win_str!("SetWindowDisplayAffinity"),
    );

    #[repr(C)]
    struct Data {
        pub affinity: u32,
        pub hwnd: usize,
        pub swda: extern "stdcall" fn(usize, u32),
        pub handled: bool,
    }
    extern "C" fn injected_func(data: &mut Data) -> u32 {
        (data.swda)(data.hwnd as _, data.affinity);
        data.handled = true;
        1
    }
    let data = Data {
        hwnd: hwnd as _,
        affinity: affinity as u32,
        swda: unsafe { std::mem::transmute(swda) },
        handled: false,
    };

    let (status, data) = inject_func(pid, injected_func, &data).unwrap();
    assert_eq!(status, 1);
    assert_eq!(data.handled, true);

    let _ = unsafe { FreeLibrary(user32) };

    Ok(())
}