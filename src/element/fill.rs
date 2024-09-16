/// Describes the style of a fill operation.
#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct FillStyle {
    /// How to fill the interior of shapes.
    pub rule: FillRule,
}

/// Describes how the interior of shapes are filled.
#[derive(Debug, Default, Clone, Copy, PartialEq, Hash)]
pub enum FillRule {
    #[default]
    NonZero,
    EvenOdd,
}

#[cfg(feature = "renderer")]
impl From<FillRule> for vello::peniko::Fill {
    fn from(fill_rule: FillRule) -> Self {
        use vello::peniko;

        match fill_rule {
            FillRule::NonZero => peniko::Fill::NonZero,
            FillRule::EvenOdd => peniko::Fill::EvenOdd,
        }
    }
}
