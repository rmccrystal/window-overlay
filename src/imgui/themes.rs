use imgui::{FontSource, FontConfig};

// https://user-images.githubusercontent.com/1657728/61432156-a1179d00-a983-11e9-87ed-f6711d7610be.png
pub fn dark_blue(imgui: &mut imgui::Context) {
    imgui.fonts().add_font(&[
        FontSource::TtfData {
            data: include_bytes!("../fonts/Ruda-Bold.ttf"),
            size_pixels: 15.0,
            config: Some({
                let mut cfg = FontConfig::default();
                cfg.name = Some("Ruda Bold".to_string());
                cfg
            })
        }
    ]);

    let style = imgui.style_mut();
    style.frame_rounding = 4.0;
    style.grab_rounding = 4.0;
    style.anti_aliased_fill = true;

    use imgui::sys::*;
    let colors = &mut style.colors;
    colors[ImGuiCol_Text as usize] = [0.95, 0.96, 0.98, 1.00];
    colors[ImGuiCol_TextDisabled as usize] = [0.36, 0.42, 0.47, 1.00];
    colors[ImGuiCol_WindowBg as usize] = [0.11, 0.15, 0.17, 1.00];
    colors[ImGuiCol_ChildBg as usize] = [0.15, 0.18, 0.22, 1.00];
    colors[ImGuiCol_PopupBg as usize] = [0.08, 0.08, 0.08, 0.94];
    colors[ImGuiCol_Border as usize] = [0.08, 0.10, 0.12, 1.00];
    colors[ImGuiCol_BorderShadow as usize] = [0.00, 0.00, 0.00, 0.00];
    colors[ImGuiCol_FrameBg as usize] = [0.20, 0.25, 0.29, 1.00];
    colors[ImGuiCol_FrameBgHovered as usize] = [0.12, 0.20, 0.28, 1.00];
    colors[ImGuiCol_FrameBgActive as usize] = [0.09, 0.12, 0.14, 1.00];
    colors[ImGuiCol_TitleBg as usize] = [0.09, 0.12, 0.14, 0.65];
    colors[ImGuiCol_TitleBgActive as usize] = [0.08, 0.10, 0.12, 1.00];
    colors[ImGuiCol_TitleBgCollapsed as usize] = [0.00, 0.00, 0.00, 0.51];
    colors[ImGuiCol_MenuBarBg as usize] = [0.15, 0.18, 0.22, 1.00];
    colors[ImGuiCol_ScrollbarBg as usize] = [0.02, 0.02, 0.02, 0.39];
    colors[ImGuiCol_ScrollbarGrab as usize] = [0.20, 0.25, 0.29, 1.00];
    colors[ImGuiCol_ScrollbarGrabHovered as usize] = [0.18, 0.22, 0.25, 1.00];
    colors[ImGuiCol_ScrollbarGrabActive as usize] = [0.09, 0.21, 0.31, 1.00];
    colors[ImGuiCol_CheckMark as usize] = [0.28, 0.56, 1.00, 1.00];
    colors[ImGuiCol_SliderGrab as usize] = [0.28, 0.56, 1.00, 1.00];
    colors[ImGuiCol_SliderGrabActive as usize] = [0.37, 0.61, 1.00, 1.00];
    colors[ImGuiCol_Button as usize] = [0.20, 0.25, 0.29, 1.00];
    colors[ImGuiCol_ButtonHovered as usize] = [0.28, 0.56, 1.00, 1.00];
    colors[ImGuiCol_ButtonActive as usize] = [0.06, 0.53, 0.98, 1.00];
    colors[ImGuiCol_Header as usize] = [0.20, 0.25, 0.29, 0.55];
    colors[ImGuiCol_HeaderHovered as usize] = [0.26, 0.59, 0.98, 0.80];
    colors[ImGuiCol_HeaderActive as usize] = [0.26, 0.59, 0.98, 1.00];
    colors[ImGuiCol_Separator as usize] = [0.20, 0.25, 0.29, 1.00];
    colors[ImGuiCol_SeparatorHovered as usize] = [0.10, 0.40, 0.75, 0.78];
    colors[ImGuiCol_SeparatorActive as usize] = [0.10, 0.40, 0.75, 1.00];
    colors[ImGuiCol_ResizeGrip as usize] = [0.26, 0.59, 0.98, 0.25];
    colors[ImGuiCol_ResizeGripHovered as usize] = [0.26, 0.59, 0.98, 0.67];
    colors[ImGuiCol_ResizeGripActive as usize] = [0.26, 0.59, 0.98, 0.95];
    colors[ImGuiCol_Tab as usize] = [0.11, 0.15, 0.17, 1.00];
    colors[ImGuiCol_TabHovered as usize] = [0.26, 0.59, 0.98, 0.80];
    colors[ImGuiCol_TabActive as usize] = [0.20, 0.25, 0.29, 1.00];
    colors[ImGuiCol_TabUnfocused as usize] = [0.11, 0.15, 0.17, 1.00];
    colors[ImGuiCol_TabUnfocusedActive as usize] = [0.11, 0.15, 0.17, 1.00];
    colors[ImGuiCol_PlotLines as usize] = [0.61, 0.61, 0.61, 1.00];
    colors[ImGuiCol_PlotLinesHovered as usize] = [1.00, 0.43, 0.35, 1.00];
    colors[ImGuiCol_PlotHistogram as usize] = [0.90, 0.70, 0.00, 1.00];
    colors[ImGuiCol_PlotHistogramHovered as usize] = [1.00, 0.60, 0.00, 1.00];
    colors[ImGuiCol_TextSelectedBg as usize] = [0.26, 0.59, 0.98, 0.35];
    colors[ImGuiCol_DragDropTarget as usize] = [1.00, 1.00, 0.00, 0.90];
    colors[ImGuiCol_NavHighlight as usize] = [0.26, 0.59, 0.98, 1.00];
    colors[ImGuiCol_NavWindowingHighlight as usize] = [1.00, 1.00, 1.00, 0.70];
    colors[ImGuiCol_NavWindowingDimBg as usize] = [0.80, 0.80, 0.80, 0.20];
    colors[ImGuiCol_ModalWindowDimBg as usize] = [0.80, 0.80, 0.80, 0.35];
}