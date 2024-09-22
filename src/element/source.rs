use std::borrow::Cow;

use crate::element::{Color, FontFamily, FontStyle, FontWeight};

/// Source of text and its properties for text operations.
#[derive(Debug, Clone, PartialEq)]
pub enum Source {
    /// Plain text.
    ///
    /// Has the same formatting for all text.
    Plain(Cow<'static, str>),
    /// Rich text.
    ///
    /// Made up of individual [`Span`]s that can override the formatting of the
    /// text.
    Rich(Vec<Span>),
}

/// Reference to a [`Source`].
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SourceRef<'a> {
    /// Plain text.
    ///
    /// Has the same formatting for all text.
    Plain(&'a str),
    /// Rich text.
    ///
    /// Made up of individual [`Span`]s that can override the formatting of the
    /// text.
    Rich(&'a [Span]),
}

/// A section of text.
#[derive(Debug, Default, Clone, PartialEq)]
pub struct Span {
    /// The text in the span.
    pub source: Cow<'static, str>,
    /// The font family of this span.
    pub font_family: Option<FontFamily>,
    /// The font style of this span.
    pub font_style: Option<FontStyle>,
    /// The font weight of this span.
    pub font_weight: Option<FontWeight>,
    /// The color of this span.
    pub color: Option<Color>,
    /// The size of this span.
    pub size: Option<f32>,
}

impl Source {
    /// The text of this source.
    pub fn text(&self) -> Cow<'static, str> {
        match self {
            Source::Plain(plain) => plain.clone(),
            Source::Rich(spans) => spans
                .iter()
                .map(|Span { source, .. }| source)
                .fold(String::new(), |mut acc, span| {
                    acc.push_str(&span);

                    acc
                })
                .into(),
        }
    }
}

impl<'a> SourceRef<'a> {
    /// The text of this source.
    pub fn text(&self) -> Cow<'a, str> {
        match self {
            Self::Plain(plain) => (*plain).into(),
            Self::Rich(spans) => spans
                .iter()
                .map(|Span { source, .. }| source)
                .fold(String::new(), |mut acc, span| {
                    acc.push_str(&span);

                    acc
                })
                .into(),
        }
    }
}

impl Span {
    /// Creates a new span from text.
    ///
    /// The returned span does not override any values.
    pub fn new(source: impl Into<Cow<'static, str>>) -> Self {
        Self { source: source.into(), ..Default::default() }
    }

    /// Returns this span with a new font family.
    pub fn with_font_family(self, font_family: FontFamily) -> Self {
        Self { font_family: Some(font_family), ..self }
    }

    /// Returns this span with a new font style.
    pub fn with_font_style(self, font_style: FontStyle) -> Self {
        Self { font_style: Some(font_style), ..self }
    }

    /// Returns this span with a new font weight.
    pub fn with_font_weight(self, font_weight: FontWeight) -> Self {
        Self { font_weight: Some(font_weight), ..self }
    }

    /// Returns this span with font style set to [`FontStyle::Italic`].
    pub fn with_italic(self) -> Self {
        self.with_font_style(FontStyle::Italic)
    }

    /// Returns this span with font weight set to [`FontWeight::BOLD`].
    pub fn with_bold(self) -> Self {
        self.with_font_weight(FontWeight::BOLD)
    }

    /// Returns this span with a new color.
    pub fn with_color(self, color: Color) -> Self {
        Self { color: Some(color), ..self }
    }

    /// Returns this span with a new size.
    pub fn with_size(self, size: f32) -> Self {
        Self { size: Some(size), ..self }
    }
}

impl From<Cow<'static, str>> for Source {
    fn from(cow: Cow<'static, str>) -> Self {
        Self::Plain(cow)
    }
}

impl From<&'static str> for Source {
    fn from(str: &'static str) -> Self {
        Self::Plain(Cow::Borrowed(str))
    }
}

impl From<String> for Source {
    fn from(string: String) -> Self {
        Self::Plain(Cow::Owned(string))
    }
}

impl From<Vec<Span>> for Source {
    fn from(spans: Vec<Span>) -> Self {
        Self::Rich(spans)
    }
}

impl<const N: usize> From<[Span; N]> for Source {
    fn from(spans: [Span; N]) -> Self {
        Self::Rich(spans.to_vec())
    }
}

impl<'a> From<&'a Source> for SourceRef<'a> {
    fn from(source: &'a Source) -> Self {
        match source {
            Source::Plain(plain) => Self::Plain(plain),
            Source::Rich(spans) => Self::Rich(spans),
        }
    }
}

impl<'a> From<&'a str> for SourceRef<'a> {
    fn from(plain: &'a str) -> Self {
        Self::Plain(plain)
    }
}

impl<'a> From<&'a [Span]> for SourceRef<'a> {
    fn from(spans: &'a [Span]) -> Self {
        Self::Rich(spans)
    }
}
