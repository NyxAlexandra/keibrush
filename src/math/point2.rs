use std::fmt;

use super::macros::*;
use super::{Infinity, Max, Min, NegOne, One, Point2, Size2, Vec2, Zero};
use crate::util::DisplayDebug;

impl<T> Point2<T> {
    /// Returns a point at the given `x` and `y`.
    pub const fn new(x: T, y: T) -> Self {
        Self { x, y }
    }

    /// Returns a point with `x` and `y` set to `v`.
    pub const fn splat(v: T) -> Self
    where
        T: Copy,
    {
        Self::new(v, v)
    }

    /// Returns a [`Vec2`] for this point.
    pub fn to_vec(self) -> Vec2<T> {
        let Self { x, y } = self;

        Vec2 { x, y }
    }

    /// Returns this point with a new `x`.
    pub fn with_x(self, x: T) -> Self {
        Self { x, ..self }
    }

    /// Returns this point with a new `y`.
    pub fn with_y(self, y: T) -> Self {
        Self { y, ..self }
    }

    /// Maps each component of this point to another.
    pub fn map<U>(self, mut f: impl FnMut(T) -> U) -> Point2<U> {
        Point2::new(f(self.x), f(self.y))
    }

    /// Returns this point with a function called on its `x`.
    pub fn map_x(self, f: impl FnOnce(T) -> T) -> Self {
        Self::new(f(self.x), self.y)
    }

    /// Returns this point with a function called on its `y`.
    pub fn map_y(self, f: impl FnOnce(T) -> T) -> Self {
        Self::new(self.x, f(self.y))
    }
}

impl<T: Zero> Zero for Point2<T> {
    const ZERO: Self = Self::splat(T::ZERO);
}

impl<T: One> One for Point2<T> {
    const ONE: Self = Self::splat(T::ONE);
}

impl<T: NegOne> NegOne for Point2<T> {
    const NEG_ONE: Self = Self::splat(T::NEG_ONE);
}

impl<T: Min> Min for Point2<T> {
    const MIN: Self = Self::splat(T::MIN);
}

impl<T: Max> Max for Point2<T> {
    const MAX: Self = Self::splat(T::MAX);
}

impl<T: Infinity> Infinity for Point2<T> {
    const INFINITY: Self = Self::splat(T::INFINITY);
}

// TODO: consider implementing `Trig`

impl<T: fmt::Debug> fmt::Debug for Point2<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_map()
            .entry(&DisplayDebug("x"), &self.x)
            .entry(&DisplayDebug("y"), &self.y)
            .finish()
    }
}

impl_add!(Point2 { x, y });
impl_sub!(Point2 { x, y });
impl_neg!(Point2 { x, y });

impl_add!(Point2 -> Size2 { x -> w, y -> h });
impl_sub!(Point2 -> Size2 { x -> w, y -> h });

impl_add!(Point2 -> Vec2 { x -> x, y -> y });
impl_sub!(Point2 -> Vec2 { x -> x, y -> y });
impl_mul!(Point2 -> Vec2 { x -> x, y -> y });
impl_div!(Point2 -> Vec2 { x -> x, y -> y });
impl_rem!(Point2 -> Vec2 { x -> x, y -> y });
impl_bitand!(Point2 -> Vec2 { x -> x, y -> y });
impl_bitor!(Point2 -> Vec2 { x -> x, y -> y });
impl_bitxor!(Point2 -> Vec2 { x -> x, y -> y });
impl_shl!(Point2 -> Vec2 { x -> x, y -> y });
impl_shr!(Point2 -> Vec2 { x -> x, y -> y });

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fmt_debug() {
        let point = Point2::new(0, 0);

        assert_eq!(&format!("{:?}", point), "{x: 0, y: 0}");
    }
}
