#[cfg(feature = "renderer")]
use vello::kurbo;

/// Describes the style of a stroke operation.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct StrokeStyle {
    /// The width of the stroke.
    pub width: f32,
    /// How to join segments of a stroke.
    pub join: Join,
    /// How to draw the start of the stroke.
    pub start: Cap,
    /// How to draw the end of the stroke.
    pub end: Cap,
    /// The miter limit for [`Join::Miter`].
    pub miter_limit: f32,
}

/// Describes how to join segments of a stroke.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Join {
    /// A straight line connecting the segments.
    Bevel,
    /// The segments are extended to their natural intersection point.
    Miter,
    /// An arc between the segments.
    #[default]
    Round,
}

/// Describes how to draw the ends of a stroke.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Cap {
    /// Flat cap.
    Butt,
    /// Square cap with dimensions equal to half the stroke width.
    Square,
    /// Rounded cap with radius equal to half the stroke width.
    #[default]
    Round,
}

impl Default for StrokeStyle {
    fn default() -> Self {
        Self {
            width: 1.0,
            join: Default::default(),
            start: Default::default(),
            end: Default::default(),
            miter_limit: 4.0,
        }
    }
}

#[cfg(feature = "renderer")]
impl From<StrokeStyle> for kurbo::Stroke {
    fn from(stroke_style: StrokeStyle) -> Self {
        let StrokeStyle { width, join, start, end, miter_limit } = stroke_style;

        kurbo::Stroke {
            width: width as _,
            join: join.into(),
            miter_limit: miter_limit as _,
            start_cap: start.into(),
            end_cap: end.into(),
            dash_pattern: kurbo::Dashes::new(),
            dash_offset: 0.0,
        }
    }
}

#[cfg(feature = "renderer")]
impl From<Join> for kurbo::Join {
    fn from(stroke_join: Join) -> Self {
        match stroke_join {
            Join::Bevel => kurbo::Join::Bevel,
            Join::Miter => kurbo::Join::Miter,
            Join::Round => kurbo::Join::Round,
        }
    }
}

#[cfg(feature = "renderer")]
impl From<Cap> for kurbo::Cap {
    fn from(stroke_cap: Cap) -> Self {
        match stroke_cap {
            Cap::Butt => kurbo::Cap::Butt,
            Cap::Square => kurbo::Cap::Square,
            Cap::Round => kurbo::Cap::Round,
        }
    }
}
