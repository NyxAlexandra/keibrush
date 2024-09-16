use std::fmt;

use super::macros::*;
use super::{Infinity, Max, Min, NegOne, One, Trig, Vec2, Zero};
use crate::util::DisplayDebug;

impl<T> Vec2<T> {
    /// Returns a vector with the given `x` and `y` components.
    pub const fn new(x: T, y: T) -> Self {
        Self { x, y }
    }

    /// Returns a vector with the given `x` and zero for `y`.
    pub const fn from_x(x: T) -> Self
    where
        T: Zero,
    {
        Self::new(x, T::ZERO)
    }

    /// Returns a vector with the given `y` and zero for `x`.
    pub const fn from_y(y: T) -> Self
    where
        T: Zero,
    {
        Self::new(T::ZERO, y)
    }

    /// Returns a vector with `x` and `y` set to `v`.
    pub const fn splat(v: T) -> Self
    where
        T: Copy,
    {
        Self::new(v, v)
    }

    /// Returns this vector with a new `x`.
    pub fn with_x(self, x: T) -> Self {
        Self { x, ..self }
    }

    /// Returns this vector with a new `y`.
    pub fn with_y(self, y: T) -> Self {
        Self { y, ..self }
    }

    /// Maps each component in this vector to another.
    pub fn map<U>(self, mut f: impl FnMut(T) -> U) -> Vec2<U> {
        Vec2::new(f(self.x), f(self.y))
    }

    /// Returns this vector with a function called on its `x`.
    pub fn map_x(self, f: impl FnOnce(T) -> T) -> Self {
        Self::new(f(self.x), self.y)
    }

    /// Returns this vector with a function called on its `y`.
    pub fn map_y(self, f: impl FnOnce(T) -> T) -> Self {
        Self::new(self.x, f(self.y))
    }

    /// Zips together each component of this vector with another.
    ///
    /// To separate back into 2 vectors, see [`Vec2::unzip`].
    pub fn zip<U>(self, other: Vec2<U>) -> Vec2<(T, U)> {
        Vec2::new((self.x, other.x), (self.y, other.y))
    }
}

impl<T, U> Vec2<(T, U)> {
    /// Unzip each component in this vector.
    pub fn unzip(self) -> (Vec2<T>, Vec2<U>) {
        let Self { x: (x_t, x_u), y: (y_t, y_u) } = self;

        (Vec2::new(x_t, y_t), Vec2::new(x_u, y_u))
    }
}

impl<T: Zero> Zero for Vec2<T> {
    const ZERO: Self = Self::splat(T::ZERO);
}

impl<T: One> One for Vec2<T> {
    const ONE: Self = Self::splat(T::ONE);
}

impl<T: NegOne> NegOne for Vec2<T> {
    const NEG_ONE: Self = Self::splat(T::NEG_ONE);
}

impl<T: Min> Min for Vec2<T> {
    const MIN: Self = Self::splat(T::MIN);
}

impl<T: Max> Max for Vec2<T> {
    const MAX: Self = Self::splat(T::MAX);
}

impl<T: Infinity> Infinity for Vec2<T> {
    const INFINITY: Self = Self::splat(T::INFINITY);
}

impl<T: Trig> Trig for Vec2<T> {
    fn sin(self) -> Self {
        self.map(T::sin)
    }

    fn cos(self) -> Self {
        self.map(T::cos)
    }

    fn tan(self) -> Self {
        self.map(T::tan)
    }

    fn sin_cos(self) -> (Self, Self) {
        self.map(T::sin_cos).unzip()
    }
}

impl<T: fmt::Debug> fmt::Debug for Vec2<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // `Vec2::new(0, 0)`: `{x: 0, y: 0}`

        f.debug_map()
            .entry(&DisplayDebug("x"), &self.x)
            .entry(&DisplayDebug("y"), &self.y)
            .finish()
    }
}

impl_add!(Vec2 { x, y });
impl_sub!(Vec2 { x, y });
impl_mul!(Vec2 { x, y });
impl_div!(Vec2 { x, y });
impl_rem!(Vec2 { x, y });
impl_neg!(Vec2 { x, y });
impl_bitand!(Vec2 { x, y });
impl_bitor!(Vec2 { x, y });
impl_bitxor!(Vec2 { x, y });
impl_shl!(Vec2 { x, y });
impl_shr!(Vec2 { x, y });
impl_not!(Vec2 { x, y });

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fmt_debug() {
        let vec = Vec2::new(1, 2);

        assert_eq!(&format!("{:?}", vec), "{x: 1, y: 2}");
    }
}
