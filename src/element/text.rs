use std::borrow::Cow;

#[cfg(feature = "renderer")]
use parley::fontique;

use super::Color;

/// Describes the style of text.
#[derive(Debug, Clone, PartialEq)]
pub struct TextStyle {
    /// The font of the text.
    pub font: Font,
    /// The color of the text (default: [`Color::WHITE`]).
    pub color: Color,
    /// The size of the text (default: `16.0`).
    pub size: f32,
    /// The alignment of the text within its bounding-box.
    pub alignment: TextAlignment,
}

/// Describes the font of text.
#[derive(Debug, Default, Clone, PartialEq)]
pub struct Font {
    /// The name of the font.
    pub family: FontFamily,
    /// Fallback fonts to use.
    pub fallback: Vec<FontFamily>,
    /// The style of the font (normal, italic).
    pub style: FontStyle,
    /// The thickness of the glyphs.
    pub weight: FontWeight,
}

/// The name of a font.
#[derive(Debug, Default, Clone, Eq, PartialEq, Hash)]
pub enum FontFamily {
    #[default]
    SansSerif,
    Serif,
    Monospace,
    Named(Cow<'static, str>),
}

/// The style of a font (normal, italic).
#[derive(Debug, Default, Clone, Copy, Eq, PartialEq, Hash)]
pub enum FontStyle {
    #[default]
    Normal,
    Italic,
}

/// The thickness of the glyphs of a font.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct FontWeight(f32);

/// How to align text within its bounding-box.
#[derive(Debug, Default, Clone, Copy, Eq, PartialEq, Hash)]
pub enum TextAlignment {
    /// Align text to the start of the bounding-box.
    #[default]
    Start,
    /// Align text to the middle of the bounding-box.
    Middle,
    /// Align text to the end of the bounding-box.
    End,
}

impl Default for TextStyle {
    fn default() -> Self {
        Self {
            font: Default::default(),
            color: Color::WHITE,
            size: 16.0,
            alignment: Default::default(),
        }
    }
}

#[cfg(feature = "renderer")]
impl<'a> From<&'a FontFamily> for parley::style::FontFamily<'a> {
    fn from(font_family: &'a FontFamily) -> Self {
        use parley::style::GenericFamily;

        match font_family {
            FontFamily::Serif => parley::style::FontFamily::Generic(GenericFamily::Serif),
            FontFamily::SansSerif => parley::style::FontFamily::Generic(GenericFamily::SansSerif),
            FontFamily::Monospace => parley::style::FontFamily::Generic(GenericFamily::Monospace),
            FontFamily::Named(name) => parley::style::FontFamily::Named(name),
        }
    }
}

#[cfg(feature = "renderer")]
impl From<FontStyle> for parley::style::FontStyle {
    fn from(font_style: FontStyle) -> Self {
        match font_style {
            FontStyle::Normal => parley::style::FontStyle::Normal,
            FontStyle::Italic => parley::style::FontStyle::Italic,
        }
    }
}

impl FontWeight {
    /// Font weight of 700.
    pub const BOLD: Self = Self(700.0);
    /// Font weight of 400.
    pub const NORMAL: Self = Self(400.0);

    /// Returns a new font for a weight value.
    pub const fn new(value: f32) -> Self {
        Self(value)
    }

    /// The value of the weight.
    pub const fn get(self) -> f32 {
        self.0
    }
}

impl Default for FontWeight {
    fn default() -> Self {
        Self::NORMAL
    }
}

#[cfg(feature = "renderer")]
impl From<FontWeight> for fontique::Weight {
    fn from(font_weight: FontWeight) -> Self {
        fontique::Weight::new(font_weight.0)
    }
}

#[cfg(feature = "renderer")]
impl From<TextAlignment> for parley::layout::Alignment {
    fn from(text_alignment: TextAlignment) -> Self {
        match text_alignment {
            TextAlignment::Start => parley::layout::Alignment::Start,
            TextAlignment::Middle => parley::layout::Alignment::Middle,
            TextAlignment::End => parley::layout::Alignment::End,
        }
    }
}
