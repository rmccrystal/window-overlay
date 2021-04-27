pub fn get_keys_down(ui: &::imgui::Ui) -> Vec<usize> {
    ui.io()
        .keys_down.iter().enumerate()
        .filter(|(i, &down)| down)
        .map(|(i, _)| i)
        .collect()
}