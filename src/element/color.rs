/// An `Srgba` color.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

impl Color {
    pub const BLACK: Self = Self::rgb(0.0, 0.0, 0.0);
    pub const BLUE: Self = Self::rgb(0.0, 0.0, 1.0);
    pub const GREEN: Self = Self::rgb(0.0, 1.0, 0.0);
    pub const RED: Self = Self::rgb(1.0, 0.0, 0.0);
    pub const TRANSPARENT: Self = Self::rgba(0.0, 0.0, 0.0, 0.0);
    pub const WHITE: Self = Self::rgb(1.0, 1.0, 1.0);

    /// Creates an opaque color from `rgb` components.
    pub const fn rgb(r: f32, g: f32, b: f32) -> Self {
        Self { r, g, b, a: 1.0 }
    }

    /// Creates a color from `rgba` components.
    pub const fn rgba(r: f32, g: f32, b: f32, a: f32) -> Self {
        Self { r, g, b, a }
    }
}

#[cfg(feature = "renderer")]
impl From<Color> for vello::peniko::Color {
    fn from(color: Color) -> Self {
        let Color { r, g, b, a } = color;

        Self::rgba(r as _, g as _, b as _, a as _)
    }
}
