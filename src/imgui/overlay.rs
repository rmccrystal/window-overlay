use std::collections::HashMap;

/// Represents a frame that be can be drawn on
pub struct OverlayWindow<'a, 'ui> {
    pub font_ids: HashMap<super::types::Font, FontId>,
    pub ui: &'a mut Ui<'ui>,
    style_token: StyleStackToken,
    color_token: ColorStackToken,
    window_token: WindowToken,
    align_to_pixel: bool
}

impl<'a, 'ui> OverlayWindow<'a, 'ui> {
    /// Creates a frame from a context
    pub fn begin(
        ui: &'a mut Ui<'ui>,
        font_ids: &HashMap<super::types::Font, FontId>,
        align_to_pixel: bool
    ) -> Self {
        let style_token = ui.push_style_vars(&[StyleVar::WindowBorderSize(0.0), StyleVar::WindowPadding([0.0, 0.0])]);
        let color_token = ui.push_style_color(StyleColor::WindowBg, [0.0, 0.0, 0.0, 0.0]);
        let window_token = Window::new(im_str!("##overlay"))
            .flags(WindowFlags::NO_TITLE_BAR | WindowFlags::NO_INPUTS)
            .position([0.0, 0.0], Condition::Always)
            .size(ui.io().display_size, Condition::Always)
            .begin(&ui).unwrap();
        Self {
            font_ids: font_ids.clone(),
            style_token,
            window_token,
            color_token,
            ui,
            align_to_pixel
        }
    }

    pub fn end(self) {
        self.window_token.end(&self.ui);
        self.style_token.pop(&self.ui);
        self.color_token.pop(&self.ui);
    }

    fn get_draw_list(&self) -> WindowDrawList {
        self.ui.get_window_draw_list()
    }
}

impl Draw for OverlayWindow<'_, '_> {
    fn draw_line(&mut self, mut p1: Vector2, mut p2: Vector2, options: LineOptions) {
        if self.align_to_pixel {
            p1 = p1.round();
            p2 = p2.round();
        }

        let draw_list = self.get_draw_list();
        draw_list
            .add_line(p1.into(), p2.into(), options.color)
            .thickness(options.width)
            .build()
    }

    fn draw_box(&mut self, mut p1: Vector2, mut p2: Vector2, options: BoxOptions) {
        if self.align_to_pixel {
            p1 = p1.round();
            p2 = p2.round();
        }

        let draw_list = self.get_draw_list();
        draw_list
            .add_rect(p1.into(), p2.into(), options.color)
            .thickness(options.width)
            .rounding(options.rounding)
            .filled(options.filled)
            .build()
    }

    fn draw_text(&mut self, mut origin: Vector2, text: &str, options: TextOptions) {
        if self.align_to_pixel {
            origin = origin.round()
        }

        let draw_list = self.get_draw_list();
        let text = unsafe { ImStr::from_ptr_unchecked(ImString::new(text).as_ptr()) };

        let font = *self.font_ids.get(&options.font).unwrap();
        let _font_size = options.font_size.unwrap_or(0.0);

        let font_token = self.ui.push_font(font);

        let x = match options.centered_horizontal {
            false => origin.x,
            true => origin.x - (self.ui.calc_text_size(text, false, 0.0)[0] / 2.0),
        };
        let y = match options.centered_vertical {
            false => origin.y,
            true => origin.y - (self.ui.calc_text_size(text, false, 0.0)[0] / 2.0),
        };

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

    fn draw_circle(&mut self, _origin: Vector2, _radius: f32, _options: CircleOptions) {
        unimplemented!()
    }
}
