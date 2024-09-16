use crate::Color;
#[cfg(feature = "renderer")]
use vello::peniko;

/// Source of pixels for a fill or stroke operation.
#[derive(Debug, Clone, PartialEq)]
pub enum Brush {
    /// A solid color fill.
    Solid(Color),
}

impl From<Color> for Brush {
    fn from(color: Color) -> Self {
        Self::Solid(color)
    }
}

#[cfg(feature = "renderer")]
impl From<Brush> for peniko::Brush {
    fn from(brush: Brush) -> Self {
        match brush {
            Brush::Solid(Color { r, g, b, a }) => {
                peniko::Brush::Solid(peniko::Color::rgba(r as _, g as _, b as _, a as _))
            },
        }
    }
}

#[cfg(feature = "renderer")]
impl From<Brush> for peniko::BrushRef<'_> {
    fn from(brush: Brush) -> Self {
        (&brush).into()
    }
}

#[cfg(feature = "renderer")]
impl From<&Brush> for peniko::BrushRef<'_> {
    fn from(brush: &Brush) -> Self {
        match brush {
            Brush::Solid(Color { r, g, b, a }) => peniko::BrushRef::Solid(
                peniko::Color::rgba(*r as _, *g as _, *b as _, *a as _),
            ),
        }
    }
}
