use ::imgui::Context;

pub mod window;
pub mod util;
pub mod imgui;
pub mod types;
pub mod shellcode;

use ::imgui::*;
use crate::imgui::Imgui;
use crate::util::get_windows;

pub fn main() {
    let mut w = window::Window::create().unwrap();
    w.controller.hide_screenshots(true);
    let target = get_windows().into_iter().find(|window| window.title == Some("Untitled - Notepad".to_string())).unwrap();
    w.controller.set_target(Some(target.hwnd));

    let mut ctx = Context::create();
    imgui::themes::dark_blue(&mut ctx);

    let imgui = Imgui::new(w, ctx);
    imgui.run(|ui, controller| {
        controller.clickthrough(true);
        ui.show_demo_window(&mut true);
    })
}
