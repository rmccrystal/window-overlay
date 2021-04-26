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
use crate::imgui::keybind::keybind_select;

pub fn main() {
    let mut w = window::OverlayWindow::create().unwrap();
    w.controller.hide_screenshots(true);
    // let target = get_windows().into_iter().find(|window| window.title == Some("*Untitled - Notepad".to_string())).unwrap();
    let target = get_windows().into_iter().find(|window| window.title == Some(r"C:\Windows\system32\cmd.exe".to_string())).unwrap();
    w.controller.set_target(Some(target.hwnd));

    let mut ctx = Context::create();
    imgui::themes::main_theme(&mut ctx);
    imgui::themes::dark_blue(&mut ctx);

    let imgui: Imgui<()> = Imgui::new(w, ctx);
    imgui.run(|ui, state, ctx| {
        if unsafe { GetAsyncKeyState(VK_F1) < 0} {
            ctx.ui_open = !ctx.ui_open;
        }
        if !ctx.ui_open {
            return;
        }

        Window::new(im_str!("Cheat"))
            .size([200.0, 300.0], Condition::FirstUseEver)
            .collapsible(false)
            .focus_on_appearing(true)
            .build(&ui, || {
            let n = ui.begin_menu_bar();

            TabBar::new(im_str!("TabBar")).build(&ui, || {
                TabItem::new(im_str!("Aimbot")).build(&ui, || {
                    ui.checkbox(im_str!("Enabled"), &mut true);
                    keybind_select(&ui, im_str!("Aimbot Key"), &mut 1);
                });
                TabItem::new(im_str!("ESP")).build(&ui, || {
                    ui.checkbox(im_str!("Enabled"), &mut true);
                });
                TabItem::new(im_str!("Misc")).build(&ui, || {
                    ui.checkbox(im_str!("Closest Player"), &mut true);
                });
            });
        });

        ui.show_demo_window(&mut true);
    })
}
