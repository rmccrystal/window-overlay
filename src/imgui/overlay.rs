use std::collections::HashMap;
use crate::imgui::RenderContext;
use ::imgui::*;
use crate::types::*;

/// Represents a frame that be can be drawn on
pub struct OverlayWindow<'a, 'b, 'ui> {
    context: &'a RenderContext,
    ui: &'b Ui<'ui>,
    style_token: StyleStackToken,
    color_token: ColorStackToken,
    window_token: WindowToken,
    align_to_pixel: bool,
}

impl<'a, 'b, 'ui> OverlayWindow<'a, 'b, 'ui> {
    /// Creates a frame from a context
    pub fn begin(
        ui: &'b imgui::Ui<'ui>,
        context: &'a RenderContext,
        align_to_pixel: bool,
    ) -> Self {
        let style_token = ui.push_style_vars(&[StyleVar::WindowBorderSize(0.0), StyleVar::WindowPadding([0.0, 0.0])]);
        let color_token = ui.push_style_color(StyleColor::WindowBg, [0.0, 0.0, 0.0, 0.0]);
        let window_token = Window::new(im_str!("##overlay"))
            .flags(WindowFlags::NO_TITLE_BAR | WindowFlags::NO_INPUTS)
            .position([0.0, 0.0], Condition::Always)
            .size(ui.io().display_size, Condition::Always)
            .begin(&ui).unwrap();
        Self { context, ui, style_token, color_token, window_token, align_to_pixel }
    }

    pub fn build(
        ui: &'b imgui::Ui<'ui>,
        context: &'a RenderContext,
        align_to_pixel: bool,
        run: impl FnOnce(&OverlayWindow)
    ) {
        let window = Self::begin(ui, context, align_to_pixel);
        run(&window);
        window.end();
    }

    pub fn end(self) {
        self.window_token.end(&self.ui);
        self.style_token.pop(&self.ui);
        self.color_token.pop(&self.ui);
    }
}

impl OverlayWindow<'_, '_, '_> {
    fn get_draw_list(&self) -> DrawListMut {
        self.ui.get_window_draw_list()
    }

    pub fn draw_line(&self, mut p1: impl Into<[f32; 2]>, mut p2: impl Into<[f32; 2]>, options: LineOptions) {
        let mut p1 = p1.into();
        let mut p2 = p2.into();
        if self.align_to_pixel {
            p1[0] = p1[0].round();
            p1[1] = p1[1].round();
            p2[0] = p2[0].round();
            p2[1] = p2[1].round();
        }

        self.get_draw_list()
            .add_line(p1.into(), p2.into(), options.color)
            .thickness(options.width)
            .build()
    }

    pub fn draw_box(&self, mut p1: impl Into<[f32; 2]>, mut p2: impl Into<[f32; 2]>, options: BoxOptions) {
        let mut p1 = p1.into();
        let mut p2 = p2.into();
        if self.align_to_pixel {
            p1[0] = p1[0].round();
            p1[1] = p1[1].round();
            p2[0] = p2[0].round();
            p2[1] = p2[1].round();
        }

        self.get_draw_list()
            .add_rect(p1.into(), p2.into(), options.color)
            .thickness(options.width)
            .rounding(options.rounding)
            .filled(options.filled)
            .build()
    }

    pub fn draw_text(&self, mut origin: impl Into<[f32; 2]>, text: &str, options: TextOptions) {
        let mut origin = origin.into();
        if self.align_to_pixel {
            origin[0] = origin[0].round();
            origin[1] = origin[1].round();
        }

        let text = unsafe { ImStr::from_ptr_unchecked(ImString::new(text).as_ptr()) };

        let font = *self.context.fonts.get(&options.font).unwrap();

        let font_token = self.ui.push_font(font);

        let x = match options.centered_horizontal {
            false => origin[0],
            true => origin[0] - (self.ui.calc_text_size(text, false, 0.0)[0] / 2.0),
        };
        let y = match options.centered_vertical {
            false => origin[1],
            true => origin[1] - (self.ui.calc_text_size(text, false, 0.0)[0] / 2.0),
        };

        let draw_list = self.get_draw_list();

        let draw = |color, offset: (f32, f32)| {
            draw_list.add_text([x + offset.0, y + offset.1], color, text);
        };

        let shadow_color = options.shadow_color;
        match options.style {
            TextStyle::Shadow => {
                draw(shadow_color, (1.0, 1.0));
            }
            TextStyle::Outlined => {
                draw(shadow_color, (1.0, 1.0));
                draw(shadow_color, (1.0, -1.0));
                draw(shadow_color, (-1.0, 1.0));
                draw(shadow_color, (-1.0, -1.0));
                draw(shadow_color, (0.0, 1.0));
                draw(shadow_color, (0.0, -1.0));
                draw(shadow_color, (1.0, 0.0));
                draw(shadow_color, (-1.0, 0.0));
            }
            TextStyle::None => {}
        }

        draw(options.color, (0.0, 0.0));

        font_token.pop(&self.ui);
    }

    fn draw_circle(&self, _origin: [f32; 2], _radius: f32, _options: CircleOptions) {
        unimplemented!()
    }
}
