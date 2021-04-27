use imgui::ImColor32;

/// Wraps u32 that represents a packed RGBA color. Mostly used by types in the
/// low level custom drawing API, such as [`DrawListMut`](crate::DrawListMut).
///
/// The bits of a color are in "`0xAABBGGRR`" format (e.g. RGBA as little endian
/// bytes). For clarity: we don't support an equivalent to the
/// `IMGUI_USE_BGRA_PACKED_COLOR` define.
///
/// This used to be named `ImColor32`, but was renamed to avoid confusion with
/// the type with that name in the C++ API (which uses 32 bits per channel).
///
/// While it doesn't provide methods to access the fields, they can be accessed
/// via the `Deref`/`DerefMut` impls it provides targeting
/// [`imgui::color::ImColor32Fields`](crate::color::ImColor32Fields), which has
/// no other meaningful uses.
///
/// # Example
/// ```
/// let mut c = imgui::ImColor32::from_rgba(0x80, 0xc0, 0x40, 0xff);
/// assert_eq!(c.to_bits(), 0xff_40_c0_80); // Note: 0xAA_BB_GG_RR
/// // Field access
/// assert_eq!(c.r, 0x80);
/// assert_eq!(c.g, 0xc0);
/// assert_eq!(c.b, 0x40);
/// assert_eq!(c.a, 0xff);
/// c.b = 0xbb;
/// assert_eq!(c.to_bits(), 0xff_bb_c0_80);
/// ```
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[repr(transparent)]
pub struct Color(pub u32);

impl Color {
    /// Convenience constant for solid black.
    pub const BLACK: Self = Self(0xff_00_00_00);

    /// Convenience constant for solid white.
    pub const WHITE: Self = Self(0xff_ff_ff_ff);

    /// Convenience constant for full transparency.
    pub const TRANSPARENT: Self = Self(0);

    /// Construct a color from 4 single-byte `u8` channel values, which should
    /// be between 0 and 255.
    #[inline]
    pub const fn from_rgba(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self(
            ((a as u32) << Self::A_SHIFT)
                | ((r as u32) << Self::R_SHIFT)
                | ((g as u32) << Self::G_SHIFT)
                | ((b as u32) << Self::B_SHIFT),
        )
    }

    /// Construct a fully opaque color from 3 single-byte `u8` channel values.
    /// Same as [`Self::from_rgba`] with a == 255
    #[inline]
    pub const fn from_rgb(r: u8, g: u8, b: u8) -> Self {
        Self::from_rgba(r, g, b, 0xff)
    }

    pub fn from_hsv(h: f32, mut s: f32, mut v: f32) -> Self {
        s /= 100.0;
        v /= 100.0;

        let mut r = 0.0;
        let mut g = 0.0;
        let mut b = 0.0;

        let hi = (h / 60.0) as i32 % 6;
        let f = (h / 60.0) - hi as f32;
        let p = v * (1.0 - s);
        let q = v * (1.0 - s * f);
        let t = v * (1.0 - s * (1.0 - f));

        match hi {
            0 => {
                r = v;
                g = t;
                b = p;
            }
            1 => {
                r = q;
                g = v;
                b = p;
            }
            2 => {
                r = p;
                g = v;
                b = t;
            }
            3 => {
                r = p;
                g = q;
                b = v;
            }
            4 => {
                r = t;
                g = p;
                b = v;
            }
            5 => {
                r = v;
                g = p;
                b = q;
            }
            _ => {}
        }

        Self::from_rgb((r * 255.0) as _, (g * 255.0) as _, (b * 255.0) as _)
    }

    /// Creates a new color from a hex value formatted like 0xRED_GREEN_BLUE_ALPHA
    /// For example: 0xFF0000 would be red
    pub const fn from_hex(mut hex: u32) -> Self {
        // If the hex doesn't include an A value, assume 0xFF
        if hex <= 0xFFFFFF {
            hex <<= 8;
            hex += 0xFF;
        }
        hex = hex.swap_bytes();
        Self(hex)
    }

    /// Construct a fully opaque color from 4 `f32` channel values in the range
    /// `0.0 ..= 1.0` (values outside this range are clamped to it, with NaN
    /// mapped to 0.0).
    ///
    /// Note: No alpha premultiplication is done, so your input should be have
    /// premultiplied alpha if needed.
    #[inline]
    // not const fn because no float math in const eval yet 😩
    pub fn from_rgba_f32s(r: f32, g: f32, b: f32, a: f32) -> Self {
        Self::from_rgba(
            f32_to_u8_sat(r),
            f32_to_u8_sat(g),
            f32_to_u8_sat(b),
            f32_to_u8_sat(a),
        )
    }

    /// Return the channels as an array of f32 in `[r, g, b, a]` order.
    #[inline]
    pub fn to_rgba_f32s(self) -> [f32; 4] {
        let &ColorFields { r, g, b, a } = &*self;
        [
            u8_to_f32_sat(r),
            u8_to_f32_sat(g),
            u8_to_f32_sat(b),
            u8_to_f32_sat(a),
        ]
    }

    /// Return the channels as an array of u8 in `[r, g, b, a]` order.
    #[inline]
    pub fn to_rgba(self) -> [u8; 4] {
        let &ColorFields { r, g, b, a } = &*self;
        [r, g, b, a]
    }

    /// Equivalent to [`Self::from_rgba_f32s`], but with an alpha of 1.0 (e.g.
    /// opaque).
    #[inline]
    pub fn from_rgb_f32s(r: f32, g: f32, b: f32) -> Self {
        Self::from_rgba(f32_to_u8_sat(r), f32_to_u8_sat(g), f32_to_u8_sat(b), 0xff)
    }

    /// Construct a color from the `u32` that makes up the bits in `0xAABBGGRR`
    /// format.
    ///
    /// Specifically, this takes the RGBA values as a little-endian u32 with 8
    /// bits per channel.
    ///
    /// Note that [`ImColor32::from_rgba`] may be a bit easier to use.
    #[inline]
    pub const fn from_bits(u: u32) -> Self {
        Self(u)
    }

    /// Return the bits of the color as a u32. These are in "`0xAABBGGRR`" format, that
    /// is, little-endian RGBA with 8 bits per channel.
    #[inline]
    pub const fn to_bits(self) -> u32 {
        self.0
    }

    // These are public in C++ ImGui, should they be public here?
    /// The number of bits to shift the byte of the red channel. Always 0.
    const R_SHIFT: u32 = 0;
    /// The number of bits to shift the byte of the green channel. Always 8.
    const G_SHIFT: u32 = 8;
    /// The number of bits to shift the byte of the blue channel. Always 16.
    const B_SHIFT: u32 = 16;
    /// The number of bits to shift the byte of the alpha channel. Always 24.
    const A_SHIFT: u32 = 24;
}

impl Default for Color {
    #[inline]
    fn default() -> Self {
        Self::TRANSPARENT
    }
}

impl std::fmt::Debug for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Color")
            .field("r", &self.r)
            .field("g", &self.g)
            .field("b", &self.b)
            .field("a", &self.a)
            .finish()
    }
}

/// A struct that exists to allow field access to [`ImColor32`]. It essentially
/// exists to be a `Deref`/`DerefMut` target and provide field access.
///
/// Note that while this is repr(C), be aware that on big-endian machines
/// (`cfg(target_endian = "big")`) the order of the fields is reversed, as this
/// is a view into a packed u32.
///
/// Generally should not be used, except as the target of the `Deref` impl of
/// [`ImColor32`].
#[derive(Copy, Clone, Debug)]
#[repr(C, align(4))]
// Should this be #[non_exhaustive] to discourage direct use?
#[rustfmt::skip]
pub struct ColorFields {
    #[cfg(target_endian = "little")] pub r: u8,
    #[cfg(target_endian = "little")] pub g: u8,
    #[cfg(target_endian = "little")] pub b: u8,
    #[cfg(target_endian = "little")] pub a: u8,
    // TODO(someday): i guess we should have BE tests, but for now I verified
    // this locally.
    #[cfg(target_endian = "big")] pub a: u8,
    #[cfg(target_endian = "big")] pub b: u8,
    #[cfg(target_endian = "big")] pub g: u8,
    #[cfg(target_endian = "big")] pub r: u8,
}

// We assume that big and little are the only endiannesses, and that exactly one
// is set. That is, PDP endian is not in use, and the we aren't using a
// completely broken custom target json or something.
#[cfg(any(
all(target_endian = "little", target_endian = "big"),
all(not(target_endian = "little"), not(target_endian = "big")),
))]
compile_error!("`cfg(target_endian = \"little\")` must be `cfg(not(target_endian = \"big\")`");

// static assert sizes match
const _: [(); core::mem::size_of::<Color>()] = [(); core::mem::size_of::<ColorFields>()];
const _: [(); core::mem::align_of::<Color>()] = [(); core::mem::align_of::<ColorFields>()];

impl core::ops::Deref for Color {
    type Target = ColorFields;
    #[inline]
    fn deref(&self) -> &ColorFields {
        // Safety: we statically assert the size and align match, and neither
        // type has any special invariants.
        unsafe { &*(self as *const Self as *const ColorFields) }
    }
}

impl core::ops::DerefMut for Color {
    #[inline]
    fn deref_mut(&mut self) -> &mut ColorFields {
        // Safety: we statically assert the size and align match, and neither
        // type has any special invariants.
        unsafe { &mut *(self as *mut Self as *mut ColorFields) }
    }
}

impl Into<::imgui::ImColor32> for Color {
    fn into(self) -> ImColor32 {
        ImColor32::from_bits(self.0)
    }
}

impl From<Color> for u32 {
    #[inline]
    fn from(color: Color) -> Self {
        color.0
    }
}

impl From<u32> for Color {
    #[inline]
    fn from(color: u32) -> Self {
        Color(color)
    }
}

impl From<[f32; 4]> for Color {
    #[inline]
    fn from(v: [f32; 4]) -> Self {
        Self::from_rgba_f32s(v[0], v[1], v[2], v[3])
    }
}

impl From<(f32, f32, f32, f32)> for Color {
    #[inline]
    fn from(v: (f32, f32, f32, f32)) -> Self {
        Self::from_rgba_f32s(v.0, v.1, v.2, v.3)
    }
}

impl From<[f32; 3]> for Color {
    #[inline]
    fn from(v: [f32; 3]) -> Self {
        Self::from_rgb_f32s(v[0], v[1], v[2])
    }
}

impl From<(f32, f32, f32)> for Color {
    fn from(v: (f32, f32, f32)) -> Self {
        Self::from_rgb_f32s(v.0, v.1, v.2)
    }
}

impl From<Color> for [f32; 4] {
    #[inline]
    fn from(v: Color) -> Self {
        v.to_rgba_f32s()
    }
}

impl From<Color> for (f32, f32, f32, f32) {
    #[inline]
    fn from(color: Color) -> Self {
        let [r, g, b, a]: [f32; 4] = color.into();
        (r, g, b, a)
    }
}

// These utilities might be worth making `pub` as free functions in
// `crate::color` so user code can ensure their numeric handling is
// consistent...

/// Clamp `v` to between 0.0 and 1.0, always returning a value between those.
///
/// Never returns NaN, or -0.0 — instead returns +0.0 for these (We differ from
/// C++ Dear ImGUI here which probably is just ignoring values like these).
#[inline]
pub(crate) fn saturate(v: f32) -> f32 {
    // Note: written strangely so that special values (NaN/-0.0) are handled
    // automatically with no extra checks.
    if v > 0.0 {
        if v <= 1.0 {
            v
        } else {
            1.0
        }
    } else {
        0.0
    }
}

/// Quantize a value in `0.0..=1.0` to `0..=u8::MAX`. Input outside 0.0..=1.0 is
/// clamped. Uses a bias of 0.5, because we assume centered quantization is used
/// (and because C++ imgui does it too). See:
/// - https://github.com/ocornut/imgui/blob/e28b51786eae60f32c18214658c15952639085a2/imgui_internal.h#L218
/// - https://cbloomrants.blogspot.com/2020/09/topics-in-quantization-for-games.html
///   (see `quantize_centered`)
#[inline]
pub(crate) fn f32_to_u8_sat(f: f32) -> u8 {
    let f = saturate(f) * 255.0 + 0.5;
    // Safety: `saturate`'s result is between 0.0 and 1.0 (never NaN even for
    // NaN input), and so for all inputs, `saturate(f) * 255.0 + 0.5` is inside
    // `0.5 ..= 255.5`.
    //
    // This is verified for all f32 in `test_f32_to_u8_sat_exhaustive`.
    //
    // Also note that LLVM doesn't bother trying to figure this out so the
    // unchecked does actually help. (That said, this likely doesn't matter
    // for imgui-rs, but I had this code in another project and it felt
    // silly to needlessly pessimize it).
    unsafe { f.to_int_unchecked() }
}

/// Opposite of `f32_to_u8_sat`. Since we assume centered quantization, this is
/// equivalent to dividing by 255 (or, multiplying by 1.0/255.0)
#[inline]
pub(crate) fn u8_to_f32_sat(u: u8) -> f32 {
    (u as f32) * (1.0 / 255.0)
}

macro_rules! static_color {
    ($name:ident,$hex:literal) => {
        // pub fn $name() -> Self {
        //     Self::from_hex($hex)
        // }
        pub const $name: Self = Self::from_hex($hex);
    };
}

// Colors from https://blueprintjs.com/docs/#core/colors
// Source: https://github.com/palantir/blueprint/tree/develop/packages/core/src/common/colors.ts
impl Color {
    static_color!(BLUE1, 0x0E5A8A);
    static_color!(BLUE2, 0x106BA3);
    static_color!(BLUE3, 0x137CBD);
    static_color!(BLUE4, 0x2B95D6);
    static_color!(BLUE5, 0x48AFF0);
    static_color!(COBALT1, 0x1F4B99);
    static_color!(COBALT2, 0x2458B3);
    static_color!(COBALT3, 0x2965CC);
    static_color!(COBALT4, 0x4580E6);
    static_color!(COBALT5, 0x669EFF);
    static_color!(DARK_GRAY1, 0x182026);
    static_color!(DARK_GRAY2, 0x202B33);
    static_color!(DARK_GRAY3, 0x293742);
    static_color!(DARK_GRAY4, 0x30404D);
    static_color!(DARK_GRAY5, 0x394B59);
    static_color!(FOREST1, 0x1D7324);
    static_color!(FOREST2, 0x238C2C);
    static_color!(FOREST3, 0x29A634);
    static_color!(FOREST4, 0x43BF4D);
    static_color!(FOREST5, 0x62D96B);
    static_color!(GOLD1, 0xA67908);
    static_color!(GOLD2, 0xBF8C0A);
    static_color!(GOLD3, 0xD99E0B);
    static_color!(GOLD4, 0xF2B824);
    static_color!(GOLD5, 0xFFC940);
    static_color!(GRAY1, 0x5C7080);
    static_color!(GRAY2, 0x738694);
    static_color!(GRAY3, 0x8A9BA8);
    static_color!(GRAY4, 0xA7B6C2);
    static_color!(GRAY5, 0xBFCCD6);
    static_color!(GREEN1, 0x0A6640);
    static_color!(GREEN2, 0x0D8050);
    static_color!(GREEN3, 0x0F9960);
    static_color!(GREEN4, 0x15B371);
    static_color!(GREEN5, 0x3DCC91);
    static_color!(INDIGO1, 0x5642A6);
    static_color!(INDIGO2, 0x634DBF);
    static_color!(INDIGO3, 0x7157D9);
    static_color!(INDIGO4, 0x9179F2);
    static_color!(INDIGO5, 0xAD99FF);
    static_color!(LIGHT_GRAY1, 0xCED9E0);
    static_color!(LIGHT_GRAY2, 0xD8E1E8);
    static_color!(LIGHT_GRAY3, 0xE1E8ED);
    static_color!(LIGHT_GRAY4, 0xEBF1F5);
    static_color!(LIGHT_GRAY5, 0xF5F8FA);
    static_color!(LIME1, 0x728C23);
    static_color!(LIME2, 0x87A629);
    static_color!(LIME3, 0x9BBF30);
    static_color!(LIME4, 0xB6D94C);
    static_color!(LIME5, 0xD1F26D);
    static_color!(ORANGE1, 0xA66321);
    static_color!(ORANGE2, 0xBF7326);
    static_color!(ORANGE3, 0xD9822B);
    static_color!(ORANGE4, 0xF29D49);
    static_color!(ORANGE5, 0xFFB366);
    static_color!(RED1, 0xA82A2A);
    static_color!(RED2, 0xC23030);
    static_color!(RED3, 0xDB3737);
    static_color!(RED4, 0xF55656);
    static_color!(RED5, 0xFF7373);
    static_color!(ROSE1, 0xA82255);
    static_color!(ROSE2, 0xC22762);
    static_color!(ROSE3, 0xDB2C6F);
    static_color!(ROSE4, 0xF5498B);
    static_color!(ROSE5, 0xFF66A1);
    static_color!(SEPIA1, 0x63411E);
    static_color!(SEPIA2, 0x7D5125);
    static_color!(SEPIA3, 0x96622D);
    static_color!(SEPIA4, 0xB07B46);
    static_color!(SEPIA5, 0xC99765);
    static_color!(TURQUOISE1, 0x008075);
    static_color!(TURQUOISE2, 0x00998C);
    static_color!(TURQUOISE3, 0x00B3A4);
    static_color!(TURQUOISE4, 0x14CCBD);
    static_color!(TURQUOISE5, 0x2EE6D6);
    static_color!(VERMILION1, 0x9E2B0E);
    static_color!(VERMILION2, 0xB83211);
    static_color!(VERMILION3, 0xD13913);
    static_color!(VERMILION4, 0xEB532D);
    static_color!(VERMILION5, 0xFF6E4A);
    static_color!(VIOLET1, 0x5C255C);
    static_color!(VIOLET2, 0x752F75);
    static_color!(VIOLET3, 0x8F398F);
    static_color!(VIOLET4, 0xA854A8);
    static_color!(VIOLET5, 0xC274C2);
}
