use imgui::sys::ImColor;
use crate::color::Color;

const DEFAULT_COLOR: Color = Color::from_hex(0xFFFFFF);

pub type Point = (f32, f32);

// Generate consuming setters with a macro
macro_rules! generate_setter {
    ($member_name:ident: $setter_type:ty) => {
        pub fn $member_name(mut self, $member_name: $setter_type) -> Self {
            self.$member_name = $member_name.into();
            self
        }
    };
}

#[derive(Debug, Clone)]
pub struct LineOptions {
    pub color: Color,
    pub width: f32,
}

impl Default for LineOptions {
    fn default() -> Self {
        LineOptions {
            color: DEFAULT_COLOR,
            width: 1.0,
        }
    }
}

impl LineOptions {
    generate_setter!(color: impl Into<Color>);
    generate_setter!(width: f32);
}

#[derive(Debug, Clone)]
pub struct BoxOptions {
    pub color: Color,
    pub rounding: f32,
    pub width: f32,
    pub filled: bool,
}

impl Default for BoxOptions {
    fn default() -> Self {
        Self {
            color: DEFAULT_COLOR,
            rounding: 0.0,
            width: 1.0,
            filled: false,
        }
    }
}

impl BoxOptions {
    generate_setter!(color: impl Into<Color>);
    generate_setter!(rounding: f32);
    generate_setter!(width: f32);
    generate_setter!(filled: bool);
}

#[derive(Debug, Clone)]
pub struct TextOptions {
    pub color: Color,
    pub font: Font,
    pub centered_horizontal: bool,
    pub centered_vertical: bool,
    pub style: TextStyle,
    pub shadow_color: Color,
}

impl Default for TextOptions {
    fn default() -> Self {
        Self {
            color: DEFAULT_COLOR,
            font: Font::Verdana,
            centered_horizontal: false,
            centered_vertical: false,
            style: TextStyle::Shadow,
            shadow_color: Color::from_rgba(20, 20, 20, 150),
        }
    }
}

impl TextOptions {
    pub fn color(mut self, color: impl Into<Color>) -> Self {
        self.color = color.into();
        self.shadow_color.a = self.color.a;
        self
    }
    generate_setter!(font: Font);
    generate_setter!(centered_horizontal: bool);
    generate_setter!(centered_vertical: bool);
    generate_setter!(style: TextStyle);
    generate_setter!(shadow_color: impl Into<Color>);
}

#[derive(Debug, Clone)]
pub enum TextStyle {
    None,
    Shadow,
    Outlined,
}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub enum Font {
    Default,
    Pixel,
    Tahoma,
    Verdana,
}

impl Default for Font {
    fn default() -> Self {
        Self::Tahoma
    }
}

#[derive(Debug, Clone)]
pub struct CircleOptions {
    pub color: Color,
    pub filled: bool,
    pub width: f32,
}

impl Default for CircleOptions {
    fn default() -> Self {
        Self {
            color: DEFAULT_COLOR,
            filled: false,
            width: 1.0,
        }
    }
}

impl CircleOptions {
    generate_setter!(filled: bool);
    generate_setter!(width: f32);
}
