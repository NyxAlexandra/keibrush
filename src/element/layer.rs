pub use vello::peniko::{BlendMode, Compose, Mix};

use super::Path;
use crate::math::Affine2;

/// Describes a layer in a [`Scene`](crate::Scene).
#[derive(Debug, Clone, PartialEq)]
pub struct Layer {
    /// The transform applied to the layer.
    pub transform: Affine2<f32>,
    /// How to blend layers.
    pub blend_mode: BlendMode,
    /// A path to clip the layer to.
    pub clip: Option<Path>,
    /// The opacity of the layer (default: `1.0`).
    pub alpha: f32,
}
