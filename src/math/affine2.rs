use super::{Affine2, Mat2, One, Vec2, Zero};

impl<T> Affine2<T> {
    /// Returns an affine transform for the given transform and translation.
    pub const fn new(transform: Mat2<T>, translation: Vec2<T>) -> Self {
        Self { transform, translation }
    }

    /// Returns an affine transform for the given transform and with a zero
    /// translation.
    pub const fn from_transform(transform: Mat2<T>) -> Self
    where
        T: Zero,
    {
        Self::new(transform, Vec2::ZERO)
    }

    /// Returns a transform that scales vectors.
    pub const fn from_scale(scale: Vec2<T>) -> Self
    where
        T: Zero,
    {
        Self::from_transform(Mat2::from_scale(scale))
    }

    /// Returns an affine transform for the given translation and with an
    /// identity transformation.
    pub const fn from_translation(translation: Vec2<T>) -> Self
    where
        T: Zero + One,
    {
        Self::new(Mat2::IDENTITY, translation)
    }

    /// Maps the transform portion of this transform.
    pub fn map_transform(self, f: impl FnOnce(Mat2<T>) -> Mat2<T>) -> Self {
        Self { transform: f(self.transform), ..self }
    }

    /// Maps the translation portion of this transform.
    pub fn map_translation(self, f: impl FnOnce(Vec2<T>) -> Vec2<T>) -> Self {
        Self { translation: f(self.translation), ..self }
    }
}

impl<T: Zero + One> Affine2<T> {
    /// The identity transform.
    pub const IDENTITY: Self = Self::new(Mat2::IDENTITY, Vec2::ZERO);
}

impl<T: Zero + One> Default for Affine2<T> {
    /// The identity transform.
    fn default() -> Self {
        Self::IDENTITY
    }
}

#[cfg(feature = "renderer")]
impl From<Affine2<f32>> for vello::kurbo::Affine {
    fn from(transform: Affine2<f32>) -> Self {
        let Affine2 { transform: Mat2 { x, y }, translation } = transform;

        vello::kurbo::Affine::new(
            [x.x, x.y, y.x, y.y, translation.x, translation.y].map(|n| n as _),
        )
    }
}
