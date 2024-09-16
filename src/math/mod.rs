mod affine2;
mod macros;
mod mat2;
mod num;
mod point2;
mod rect;
mod size2;
mod vec2;

pub use self::num::*;

/// A vector with 2 components.
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Vec2<T> {
    pub x: T,
    pub y: T,
}

/// A 2x2 column-wise matrix.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Mat2<T> {
    pub x: Vec2<T>,
    pub y: Vec2<T>,
}

/// An 2d affine transform matrix.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Affine2<T> {
    pub transform: Mat2<T>,
    pub translation: Vec2<T>,
}

/// A point in 2 dimensions.
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Point2<T> {
    pub x: T,
    pub y: T,
}

/// A size in 2 dimensions.
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Size2<T> {
    pub w: T,
    pub h: T,
}

/// A rectangle defined by its origin and size.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Rect<T> {
    /// The upper-left corner of the rectangle.
    pub origin: Point2<T>,
    /// The width and height of the rectangle (`y`-down).
    pub size: Size2<T>,
}
