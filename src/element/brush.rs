#[cfg(feature = "renderer")]
use vello::peniko;
#[cfg(feature = "renderer")]
pub use vello::peniko::Extend;

use crate::{Color, Point2, Zero};

/// Source of pixels for a fill or stroke operation.
#[derive(Debug, Clone, PartialEq)]
pub enum Brush {
    /// A solid color fill.
    Solid(Color),
    // Fill with a linear gradient.
    LinearGradient(LinearGradient),
}

/// A linear gradient.
#[derive(Debug, Clone, PartialEq)]
pub struct LinearGradient {
    /// The start of the gradient.
    pub start: Point2<f32>,
    /// The end of the gradient.
    pub end: Point2<f32>,
    /// How to extend the gradient to fit the painting area.
    pub extend: Extend,
    /// The colors in the gradient.
    pub stops: Vec<ColorStop>,
}

/// A color stop in a gradient.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ColorStop {
    /// Offset into the gradient.
    pub offset: f32,
    /// The color at the offset.
    pub color: Color,
}

impl From<Color> for Brush {
    fn from(color: Color) -> Self {
        Self::Solid(color)
    }
}

impl From<LinearGradient> for Brush {
    fn from(linear_gradient: LinearGradient) -> Self {
        Self::LinearGradient(linear_gradient)
    }
}

#[cfg(feature = "renderer")]
impl From<Brush> for peniko::Brush {
    fn from(brush: Brush) -> Self {
        match brush {
            Brush::Solid(color) => peniko::Brush::Solid(color.into()),
            Brush::LinearGradient(LinearGradient { start, end, extend, stops }) => {
                peniko::Brush::Gradient(peniko::Gradient {
                    kind: peniko::GradientKind::Linear {
                        start: start.into(),
                        end: end.into(),
                    },
                    extend,
                    stops: peniko::ColorStops::from_iter(
                        stops.into_iter().map(Into::into),
                    ),
                })
            },
        }
    }
}

impl Default for LinearGradient {
    fn default() -> Self {
        Self {
            start: Point2::ZERO,
            end: Point2::ZERO,
            extend: Default::default(),
            stops: Default::default(),
        }
    }
}

impl ColorStop {
    /// Creates a new color stop from its offset and color.
    pub const fn new(offset: f32, color: Color) -> Self {
        Self { offset, color }
    }
}

#[cfg(feature = "renderer")]
impl From<ColorStop> for peniko::ColorStop {
    fn from(color_stop: ColorStop) -> Self {
        let ColorStop { offset, color } = color_stop;

        peniko::ColorStop { offset, color: color.into() }
    }
}
