use ::imgui::Context;

pub mod window;
pub mod util;
pub mod imgui;
pub mod types;
pub mod color;

use ::imgui::*;
use crate::imgui::Imgui;
use winapi::um::winuser::{GetAsyncKeyState, VK_F1, VK_LBUTTON};
use crate::imgui::keybind::keybind_select;
use winutil::get_windows;
use crate::imgui::overlay::OverlayWindow;
use crate::types::*;

pub fn main() {
    let mut w = window::OverlayWindow::create().unwrap();
    w.controller.hide_screenshots(true);
    // let target = get_windows().into_iter().find(|window| window.title == Some("*Untitled - Notepad".to_string())).unwrap();
    let target = get_windows().into_iter().find(|window| window.title == Some(r"C:\Windows\system32\cmd.exe".to_string())).unwrap();
    w.controller.set_target(Some(target.hwnd));

    let mut ctx = Context::create();
    imgui::themes::main_theme(&mut ctx);
    imgui::themes::dark_blue(&mut ctx);

    let mut aimbot_key = VK_LBUTTON;
    let mut color = [0.0, 0.0, 0.0, 0.0];

    let imgui: Imgui = Imgui::new(w, ctx);
    imgui.run(move |ui, state, ctx| {
        OverlayWindow::build(&ui, &ctx, false, |overlay| {
            overlay.draw_line([5.0, 5.0], ui.io().mouse_pos, LineOptions::default());
        });

        ctx.bypass_screenshots = false;
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
            .always_auto_resize(true)
            .build(&ui, || {
            let n = ui.begin_menu_bar();

            TabBar::new(im_str!("TabBar")).build(&ui, || {
                TabItem::new(im_str!("Aimbot")).build(&ui, || {
                    ui.checkbox(im_str!("Enabled"), &mut true);
                    ui.text(format!("{:?}", color));
                    keybind_select(&ui, state, im_str!("Aimbot Key"), &mut aimbot_key);
                    ui.checkbox(im_str!("Aim at teammates"), &mut true);
                    ComboBox::new(im_str!("Bone")).build(&ui, || {
                        Selectable::new(im_str!("Head")).build(&ui);
                        Selectable::new(im_str!("Chest")).build(&ui);
                    })
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
