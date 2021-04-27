use imgui::{ImStr, StyleColor, StyleVar};
use crate::imgui::RenderState;
use imgui::*;
use winapi::um::winuser::GetAsyncKeyState;
use crate::imgui::util::get_keys_down;

pub fn keybind_select(ui: &::imgui::Ui, render_state: &mut RenderState, title: &ImStr, key: &mut i32) {
    let listening = render_state.get(&format!("keybind:{:X}", key as *mut _ as usize), false);

    let title = if *listening { im_str!("Press key").to_owned() } else { im_str!("{}: {}", title, *key) };

    if ui.button(&title, [120.0, 20.0]) {
        *listening = !*listening
    }
    if *listening {
        let keys_down = get_keys_down(&ui);
        if let Some(key_pressed) = keys_down.get(0) {
            *key = *key_pressed as _;
            *listening = false;
        }
    }
}