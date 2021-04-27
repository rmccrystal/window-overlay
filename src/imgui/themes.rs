use imgui::{FontSource, FontConfig};
use imgui::sys::*;

pub fn main_theme(imgui: &mut imgui::Context) {
    imgui.fonts().add_font(&[
        FontSource::TtfData {
            data: include_bytes!("../fonts/Ruda-Bold.ttf"),
            size_pixels: 15.0,
            config: Some(FontConfig{
                name: Some("Ruda Bold".to_string()),
                ..Default::default()
            }),
        }
    ]);
    imgui.fonts().add_font(&[
        FontSource::TtfData {
            data: include_bytes!("../fonts/Apercu Medium.ttf"),
            size_pixels: 15.0,
            config: Some(FontConfig{
                name: Some("Apercu Medium".to_string()),
                ..Default::default()
            }),
        }
    ]);
    imgui.fonts().add_font(&[
        FontSource::TtfData {
            data: include_bytes!("../fonts/Apercu Bold.ttf"),
            size_pixels: 15.0,
            config: Some(FontConfig{
                name: Some("Apercu Bold".to_string()),
                ..Default::default()
            }),
        }
    ]);

    let style = imgui.style_mut();
    style.frame_rounding = 2.0;
    style.window_title_align = [0.5, 0.5];
    // style.grab_rounding = 4.0;
    // style.alpha = 1.0;
}

// https://user-images.githubusercontent.com/1657728/61432156-a1179d00-a983-11e9-87ed-f6711d7610be.png
pub fn dark_blue(imgui: &mut imgui::Context) {
    let style = imgui.style_mut();
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

// https://user-images.githubusercontent.com/1434736/104601393-7ee70400-567a-11eb-923d-6e9693f9c8.png
pub fn dark_red(imgui: &mut imgui::Context) {
    let style = imgui.style_mut();

    let colors = &mut style.colors;

    colors[ImGuiCol_Text as usize] = [0.75, 0.75, 0.75, 1.0];
    colors[ImGuiCol_TextDisabled as usize] = [0.35, 0.35, 0.35, 1.0];
    colors[ImGuiCol_WindowBg as usize] = [0.08, 0.08, 0.08, 1.0];
    colors[ImGuiCol_ChildBg as usize] = [0.08, 0.08, 0.08, 1.0];
    colors[ImGuiCol_PopupBg as usize] = [0.10, 0.10, 0.10, 1.0];
    colors[ImGuiCol_Border as usize] = [0.00, 0.00, 0.00, 1.0];
    colors[ImGuiCol_BorderShadow as usize] = [0.00, 0.00, 0.00, 1.0];
    colors[ImGuiCol_FrameBg as usize] = [0.00, 0.00, 0.00, 1.0];
    colors[ImGuiCol_FrameBgHovered as usize] = [0.37, 0.14, 0.14, 1.0];
    colors[ImGuiCol_FrameBgActive as usize] = [0.39, 0.20, 0.20, 1.0];
    colors[ImGuiCol_TitleBg as usize] = [0.04, 0.04, 0.04, 1.0];
    colors[ImGuiCol_TitleBgActive as usize] = [0.48, 0.16, 0.16, 1.0];
    colors[ImGuiCol_TitleBgCollapsed as usize] = [0.48, 0.16, 0.16, 1.0];
    colors[ImGuiCol_MenuBarBg as usize] = [0.14, 0.14, 0.14, 1.0];
    colors[ImGuiCol_ScrollbarBg as usize] = [0.02, 0.02, 0.02, 1.0];
    colors[ImGuiCol_ScrollbarGrab as usize] = [0.31, 0.31, 0.31, 1.0];
    colors[ImGuiCol_ScrollbarGrabHovered as usize] = [0.41, 0.41, 0.41, 1.0];
    colors[ImGuiCol_ScrollbarGrabActive as usize] = [0.51, 0.51, 0.51, 1.0];
    colors[ImGuiCol_CheckMark as usize] = [0.56, 0.10, 0.10, 1.0];
    colors[ImGuiCol_SliderGrab as usize] = [1.00, 0.19, 0.19, 1.0];
    colors[ImGuiCol_SliderGrabActive as usize] = [0.89, 0.00, 0.19, 1.0];
    colors[ImGuiCol_Button as usize] = [1.00, 0.19, 0.19, 1.0];
    colors[ImGuiCol_ButtonHovered as usize] = [0.80, 0.17, 0.00, 1.0];
    colors[ImGuiCol_ButtonActive as usize] = [0.89, 0.00, 0.19, 1.0];
    colors[ImGuiCol_Header as usize] = [0.33, 0.35, 0.36, 1.0];
    colors[ImGuiCol_HeaderHovered as usize] = [0.76, 0.28, 0.44, 1.0];
    colors[ImGuiCol_HeaderActive as usize] = [0.47, 0.47, 0.47, 1.0];
    colors[ImGuiCol_Separator as usize] = [0.32, 0.32, 0.32, 1.0];
    colors[ImGuiCol_SeparatorHovered as usize] = [0.32, 0.32, 0.32, 1.0];
    colors[ImGuiCol_SeparatorActive as usize] = [0.32, 0.32, 0.32, 1.0];
    colors[ImGuiCol_ResizeGrip as usize] = [1.00, 1.00, 1.00, 1.0];
    colors[ImGuiCol_ResizeGripHovered as usize] = [1.00, 1.00, 1.00, 1.0];
    colors[ImGuiCol_ResizeGripActive as usize] = [1.00, 1.00, 1.00, 1.0];
    colors[ImGuiCol_Tab as usize] = [0.07, 0.07, 0.07, 1.0];
    colors[ImGuiCol_TabHovered as usize] = [0.86, 0.23, 0.43, 1.0];
    colors[ImGuiCol_TabActive as usize] = [0.19, 0.19, 0.19, 1.0];
    colors[ImGuiCol_TabUnfocused as usize] = [0.05, 0.05, 0.05, 1.0];
    colors[ImGuiCol_TabUnfocusedActive as usize] = [0.13, 0.13, 0.13, 1.0];
    colors[ImGuiCol_PlotLines as usize] = [0.61, 0.61, 0.61, 1.0];
    colors[ImGuiCol_PlotLinesHovered as usize] = [1.00, 0.43, 0.35, 1.0];
    colors[ImGuiCol_PlotHistogram as usize] = [0.90, 0.70, 0.00, 1.0];
    colors[ImGuiCol_PlotHistogramHovered as usize] = [1.00, 0.60, 0.00, 1.0];
    colors[ImGuiCol_TableHeaderBg as usize] = [0.19, 0.19, 0.20, 1.0];
    colors[ImGuiCol_TableBorderStrong as usize] = [0.31, 0.31, 0.35, 1.0];
    colors[ImGuiCol_TableBorderLight as usize] = [0.23, 0.23, 0.25, 1.0];
    colors[ImGuiCol_TableRowBg as usize] = [0.00, 0.00, 0.00, 1.0];
    colors[ImGuiCol_TableRowBgAlt as usize] = [1.00, 1.00, 1.00, 1.0];
    colors[ImGuiCol_TextSelectedBg as usize] = [0.26, 0.59, 0.98, 1.0];
    colors[ImGuiCol_DragDropTarget as usize] = [1.00, 1.00, 0.00, 1.0];
    colors[ImGuiCol_NavHighlight as usize] = [0.26, 0.59, 0.98, 1.0];
    colors[ImGuiCol_NavWindowingHighlight as usize] = [1.00, 1.00, 1.00, 1.0];
    colors[ImGuiCol_NavWindowingDimBg as usize] = [0.80, 0.80, 0.80, 1.0];
    colors[ImGuiCol_ModalWindowDimBg as usize] = [0.80, 0.80, 0.80, 1.0];
}