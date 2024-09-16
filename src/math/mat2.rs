use std::fmt;

use super::{Infinity, Mat2, Max, Min, NegOne, One, Trig, Vec2, Zero};

impl<T> Mat2<T> {
    /// Returns a matrix for its columns.
    pub const fn from_columns(x: Vec2<T>, y: Vec2<T>) -> Self {
        Self { x, y }
    }

    /// Returns a matrix that can scale vectors.
    pub const fn from_scale(scale: Vec2<T>) -> Self
    where
        T: Zero,
    {
        Self::from_columns(Vec2::from_x(scale.x), Vec2::from_y(scale.y))
    }

    /// Returns a matrix with columns set to `v`.
    pub const fn splat(v: Vec2<T>) -> Self
    where
        T: Copy,
    {
        Self::from_columns(v, v)
    }

    /// Returns this matrix with a new `x` column.
    pub fn with_x(self, x: Vec2<T>) -> Self {
        Self { x, ..self }
    }

    /// Returns this matrix with a new `y` column.
    pub fn with_y(self, y: Vec2<T>) -> Self {
        Self { y, ..self }
    }

    /// Maps the columns of the matrix to another.
    pub fn map<U>(self, mut f: impl FnMut(Vec2<T>) -> Vec2<U>) -> Mat2<U> {
        Mat2::from_columns(f(self.x), f(self.y))
    }

    /// Returns this matrix with a function called on its `x` column.
    pub fn map_x(self, f: impl FnOnce(Vec2<T>) -> Vec2<T>) -> Self {
        Self::from_columns(f(self.x), self.y)
    }

    /// Returns this matrix with a function called on its `y` column.
    pub fn map_y(self, f: impl FnOnce(Vec2<T>) -> Vec2<T>) -> Self {
        Self::from_columns(self.x, f(self.y))
    }

    /// Zips together each column of this matrix and another.
    ///
    /// To separate back into to matrices, see [`Mat2::unzip`].
    pub fn zip<U>(self, other: Mat2<U>) -> Mat2<(T, U)> {
        Mat2::from_columns(self.x.zip(other.x), self.y.zip(other.y))
    }
}

impl<T, U> Mat2<(T, U)> {
    /// Unzips this matrix back into 2 matrices.
    pub fn unzip(self) -> (Mat2<T>, Mat2<U>) {
        let ((x_t, x_u), (y_t, y_u)) = (self.x.unzip(), self.y.unzip());

        (Mat2::from_columns(x_t, y_t), Mat2::from_columns(x_u, y_u))
    }
}

impl<T: Zero + One> Mat2<T> {
    /// The identity matrix.
    pub const IDENTITY: Self =
        Self::from_columns(Vec2::new(T::ONE, T::ZERO), Vec2::new(T::ZERO, T::ONE));
}

impl<T: Zero> Zero for Mat2<T> {
    const ZERO: Self = Self::splat(Vec2::splat(T::ZERO));
}

impl<T: One> One for Mat2<T> {
    const ONE: Self = Self::splat(Vec2::splat(T::ONE));
}

impl<T: NegOne> NegOne for Mat2<T> {
    const NEG_ONE: Self = Self::splat(Vec2::splat(T::NEG_ONE));
}

impl<T: Min> Min for Mat2<T> {
    const MIN: Self = Self::splat(Vec2::splat(T::MIN));
}

impl<T: Max> Max for Mat2<T> {
    const MAX: Self = Self::splat(Vec2::splat(T::MAX));
}

impl<T: Infinity> Infinity for Mat2<T> {
    const INFINITY: Self = Self::splat(Vec2::splat(T::INFINITY));
}

impl<T: Trig> Trig for Mat2<T> {
    fn sin(self) -> Self {
        self.map(Vec2::sin)
    }

    fn cos(self) -> Self {
        self.map(Vec2::cos)
    }

    fn tan(self) -> Self {
        self.map(Vec2::tan)
    }

    fn sin_cos(self) -> (Self, Self) {
        self.map(|c| {
            let (sin, cos) = c.sin_cos();

            sin.zip(cos)
        })
        .unzip()
    }
}

impl<T: fmt::Debug> fmt::Debug for Mat2<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        struct DebugVec2<'a, T>(&'a Vec2<T>);

        impl<T: fmt::Debug> fmt::Debug for DebugVec2<'_, T> {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                f.debug_list().entries([&self.0.x, &self.0.y]).finish()
            }
        }

        f.debug_list().entries([DebugVec2(&self.x), DebugVec2(&self.y)]).finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fmt_debug() {
        let mat = Mat2::from_columns(Vec2::new(1, 2), Vec2::new(3, 4));

        assert_eq!(&format!("{:?}", mat), "[[1, 2], [3, 4]]");
    }
}
