use imgui::ImStr;

pub fn keybind_select(ui: &::imgui::Ui, title: &ImStr, key: &mut i32) {
    if ui.button(title, [80.0, 20.0]) {
        ui.text("hello");
    }
}