use std::fmt;

use super::macros::*;
use super::{Infinity, Max, Min, NegOne, One, Size2, Vec2, Zero};
use crate::util::DisplayDebug;

impl<T> Size2<T> {
    /// Returns a new size for the given width and height.
    pub const fn new(w: T, h: T) -> Self {
        Self { w, h }
    }

    /// Returns a size with the width and height set to `v`.
    pub const fn splat(v: T) -> Self
    where
        T: Copy,
    {
        Self::new(v, v)
    }

    /// Returns a [`Vec2`] for this size.
    pub fn to_vec(self) -> Vec2<T> {
        let Self { w, h } = self;

        Vec2::new(w, h)
    }

    /// Returns this size with a new width.
    pub fn with_w(self, w: T) -> Self {
        Self { w, ..self }
    }

    /// Returns this size with a new width.
    pub fn with_h(self, h: T) -> Self {
        Self { h, ..self }
    }

    /// Maps the components of this size to another.
    pub fn map<U>(self, mut f: impl FnMut(T) -> U) -> Size2<U> {
        Size2::new(f(self.w), f(self.h))
    }

    /// Returns this size with a function called on its width.
    pub fn map_w(self, f: impl FnOnce(T) -> T) -> Self {
        Self::new(f(self.w), self.h)
    }

    /// Returns this size with a function called on its height.
    pub fn map_h(self, f: impl FnOnce(T) -> T) -> Self {
        Self::new(self.w, f(self.h))
    }

    /// Zips together the width and size of this size and another.
    ///
    /// To separate back into two sizes, see [`Size2::unzip`].
    pub fn zip<U>(self, other: Size2<U>) -> Size2<(T, U)> {
        Size2::new((self.w, other.w), (self.h, other.h))
    }
}

impl<T, U> Size2<(T, U)> {
    /// Unzips the tuples into two sizes.
    pub fn unzip(self) -> (Size2<T>, Size2<U>) {
        let Self { w: (w_t, w_u), h: (h_t, h_u) } = self;

        (Size2::new(w_t, h_t), Size2::new(w_u, h_u))
    }
}

impl<T: Zero> Zero for Size2<T> {
    const ZERO: Self = Self::splat(T::ZERO);
}

impl<T: One> One for Size2<T> {
    const ONE: Self = Self::splat(T::ONE);
}

impl<T: NegOne> NegOne for Size2<T> {
    const NEG_ONE: Self = Self::splat(T::NEG_ONE);
}

impl<T: Min> Min for Size2<T> {
    const MIN: Self = Self::splat(T::MIN);
}

impl<T: Max> Max for Size2<T> {
    const MAX: Self = Self::splat(T::MAX);
}

impl<T: Infinity> Infinity for Size2<T> {
    const INFINITY: Self = Self::splat(T::INFINITY);
}

impl<T: fmt::Debug> fmt::Debug for Size2<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // `Size2::new(0, 0)`: `{w: 0, h: 0}`

        f.debug_map()
            .entry(&DisplayDebug("w"), &self.w)
            .entry(&DisplayDebug("h"), &self.h)
            .finish()
    }
}

impl_add!(Size2 { w, h });
impl_sub!(Size2 { w, h });
impl_neg!(Size2 { w, h });

impl_add!(Size2 -> Vec2 { w -> x, h -> y });
impl_sub!(Size2 -> Vec2 { w -> x, h -> y });
impl_mul!(Size2 -> Vec2 { w -> x, h -> y });
impl_div!(Size2 -> Vec2 { w -> x, h -> y });
impl_rem!(Size2 -> Vec2 { w -> x, h -> y });
impl_bitand!(Size2 -> Vec2 { w -> x, h -> y });
impl_bitor!(Size2 -> Vec2 { w -> x, h -> y });
impl_bitxor!(Size2 -> Vec2 { w -> x, h -> y });
impl_shl!(Size2 -> Vec2 { w -> x, h -> y });
impl_shr!(Size2 -> Vec2 { w -> x, h -> y });

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fmt_debug() {
        let size = Size2::new(0, 0);

        assert_eq!(&format!("{:?}", size), "{w: 0, h: 0}");
    }
}
