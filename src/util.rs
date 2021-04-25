use winapi::um::winuser;
use winapi::shared::windef::HWND;
use std::ffi::c_void;
use std::mem;
use winapi::shared::minwindef::{LPARAM, BOOL};
use winapi::um::winuser::{EnumWindows, GetWindowThreadProcessId};
use log::*;
use winapi::um::libloaderapi::{LoadLibraryA, GetProcAddress, FreeLibrary};
use crate::shellcode::inject_func;
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


pub fn get_windows() -> Vec<Window> {
    let mut windows = Vec::new();
    enumerate_windows(|hwnd| {
        unsafe {
            let mut buf: Vec<u8> = vec![0; 256];

            let class = winuser::GetClassNameA(hwnd, buf.as_mut_ptr() as _, 256);
            let class = u8_to_string(&buf).unwrap();
            let class_bytes = buf.iter().map(|n| *n).take_while(|&n| n != 0).collect::<Vec<_>>();

            winuser::GetWindowTextA(hwnd, buf.as_mut_ptr() as _, 256);
            let title = u8_to_string(&buf);
            let title_bytes = buf.iter().map(|n| *n).take_while(|&n| n != 0).collect::<Vec<_>>();

            let mut pid = 0;
            winuser::GetWindowThreadProcessId(hwnd, &mut pid);

            windows.push(Window {
                hwnd,
                class,
                class_bytes,
                pid,
                title,
                title_bytes,
            })
        }
        true
    });
    windows
}

#[derive(Clone, Debug)]
pub struct Window {
    pub class: String,
    pub class_bytes: Vec<u8>,
    pub title: Option<String>,
    pub title_bytes: Vec<u8>,
    pub hwnd: HWND,
    pub pid: u32,
}

fn enumerate_windows<F>(mut callback: F)
    where F: FnMut(HWND) -> bool
{
    let mut trait_obj: &mut dyn FnMut(HWND) -> bool = &mut callback;
    let closure_pointer_pointer: *mut c_void = unsafe { mem::transmute(&mut trait_obj) };

    let lparam = closure_pointer_pointer as LPARAM;
    unsafe { EnumWindows(Some(enumerate_callback), lparam) };
}

// To continue enumeration, the callback function must return TRUE; to stop enumeration, it must return FALSE.
unsafe extern "system" fn enumerate_callback(hwnd: HWND, lparam: LPARAM) -> BOOL {
    let closure: &mut &mut dyn FnMut(HWND) -> bool = mem::transmute(lparam as *mut c_void);
    if closure(hwnd) { 1 } else { 0 }
}

fn u8_to_string(src: &[u8]) -> Option<String> {
    let nul_range_end = src.iter()
        .position(|&c| c == b'\0')
        .unwrap_or(src.len()); // default to length if no `\0` present
    String::from_utf8(src[0..nul_range_end].to_vec()).ok()
}