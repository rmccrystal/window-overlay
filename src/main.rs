use ::imgui::Context;

pub mod window;
pub mod util;
pub mod imgui;
pub mod types;
pub mod shellcode;

use ::imgui::*;
use crate::imgui::Imgui;
use crate::util::get_windows;
use winapi::um::winuser::{GetAsyncKeyState, VK_F1};

pub fn main() {
    let mut w = window::Window::create().unwrap();
    w.controller.hide_screenshots(true);
    // let target = get_windows().into_iter().find(|window| window.title == Some("*Untitled - Notepad".to_string())).unwrap();
    let target = get_windows().into_iter().find(|window| window.title == Some(r"C:\Windows\system32\cmd.exe".to_string())).unwrap();
    w.controller.set_target(Some(target.hwnd));

    let mut ctx = Context::create();
    imgui::themes::main_theme(&mut ctx);
    // imgui::themes::dark_red(&mut ctx);

    let imgui = Imgui::new(w, ctx);
    imgui.run(|ui, ctx| {
        if unsafe { GetAsyncKeyState(VK_F1) != 0} {
            ctx.ui_open = !ctx.ui_open;
        }
        if !ctx.ui_open {
            return;
        }
        ui.checkbox(im_str!("UI enabled"), &mut ctx.ui_open);
        ui.show_demo_window(&mut true);
    })
}
