use imgui::{Context, FontSource, FontId};
use std::collections::HashMap;
use crate::types;

pub fn add_fonts(ctx: &mut Context) -> HashMap<types::Font, FontId> {
    let mut fonts = HashMap::new();

    // First font is default
    fonts.insert(
        types::Font::Default,
        ctx.fonts()
            .add_font(&[FontSource::DefaultFontData { config: None }]),
    );
    fonts.insert(
        types::Font::Pixel,
        ctx.fonts().add_font(&[FontSource::TtfData {
            data: include_bytes!("../fonts/smallest_pixel-7.ttf"),
            size_pixels: 10.0,
            config: None,
        }]),
    );
    fonts.insert(
        types::Font::Verdana,
        ctx.fonts().add_font(&[FontSource::TtfData {
            data: include_bytes!("../fonts/Verdana.ttf"),
            size_pixels: 13.0,
            config: None,
        }]),
    );
    fonts.insert(
        types::Font::Tahoma,
        ctx.fonts().add_font(&[FontSource::TtfData {
            data: include_bytes!("../fonts/Tahoma.ttf"),
            size_pixels: 14.0,
            config: None,
        }]),
    );

    fonts
}
