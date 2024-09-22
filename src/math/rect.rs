use std::ops::{Add, Sub};

use super::{Max, Min, Point2, Rect, Size2, Vec2, Zero};
use crate::element::Path;

impl<T> Rect<T> {
    /// Returns a rectangle from its upper-left point and size (`x` -> right,
    /// `y` -> down).
    pub const fn new(origin: Point2<T>, size: Size2<T>) -> Self {
        Self { origin, size }
    }

    /// Returns a rectangle with zero size at an origin.
    pub fn from_origin(origin: Point2<T>) -> Self
    where
        T: Zero,
    {
        Self::new(origin, Size2::ZERO)
    }

    /// Returns a rectangle at the origin with a given size.
    pub fn from_size(size: Size2<T>) -> Self
    where
        T: Zero,
    {
        Self::new(Point2::ZERO, size)
    }

    /// Returns `self` with a new origin.
    pub fn with_origin(self, origin: Point2<T>) -> Self {
        Self { origin, ..self }
    }

    /// Returns `self` with a new size.
    pub fn with_size(self, size: Size2<T>) -> Self {
        Self { size, ..self }
    }

    /// Returns the left edge of this rectangle.
    pub fn left(self) -> T {
        self.origin.x
    }

    /// Returns the right edge of this rectangle.
    pub fn right(self) -> T::Output
    where
        T: Add,
    {
        self.origin.x + self.size.w
    }

    /// Returns the top edge of this rectangle.
    pub fn top(self) -> T {
        self.origin.y
    }

    /// Returns the right edge of this rectangle.
    pub fn bottom(self) -> T::Output
    where
        T: Add,
    {
        self.origin.y + self.size.h
    }

    /// Returns the upper-left corner of this rectangle.
    pub fn top_left(self) -> Point2<T>
    where
        T: Add<Output = T> + Copy,
    {
        Point2::new(self.left(), self.top())
    }

    /// Returns the upper-right corner of this rectangle.
    pub fn top_right(self) -> Point2<T>
    where
        T: Add<Output = T> + Copy,
    {
        Point2::new(self.right(), self.top())
    }

    /// Returns the lower-left corner of this rectangle.
    pub fn bottom_left(self) -> Point2<T>
    where
        T: Add<Output = T> + Copy,
    {
        Point2::new(self.left(), self.bottom())
    }

    /// Returns the lower-right corner of this rectangle.
    pub fn bottom_right(self) -> Point2<T>
    where
        T: Add<Output = T> + Copy,
    {
        Point2::new(self.right(), self.bottom())
    }

    /// Returns an array of the corners of this rectangle ([`Rect::top_left`],
    /// [`Rect::top_right`], [`Rect::bottom_left`], and [`Rect::bottom_right`]).
    pub fn corners(self) -> [Point2<T>; 4]
    where
        T: Add<Output = T> + Copy,
    {
        [self.top_left(), self.top_right(), self.bottom_left(), self.bottom_right()]
    }

    /// Returns this rectangle after applying an inset to it.
    pub fn with_insets(self, insets: Vec2<T>) -> Rect<T>
    where
        T: Add<Output = T> + Sub<Output = T> + Copy,
    {
        Rect { origin: self.origin + insets, size: self.size - (insets + insets) }
    }

    /// Returns `true` if this rectangle fully contains `other`.
    pub fn contains(self, other: Self) -> bool
    where
        T: PartialOrd + Add<Output = T> + Copy,
    {
        other.left() >= self.left()
            && other.right() <= self.right()
            && other.top() >= self.top()
            && other.bottom() <= self.bottom()
    }

    /// Returns `true` if `point` is within the bounds of this rectangle.
    pub fn contains_point(self, point: Point2<T>) -> bool
    where
        T: PartialOrd + Add<Output = T> + Copy,
    {
        point.x >= self.left()
            && point.x <= self.right()
            && point.y >= self.top()
            && point.y <= self.bottom()
    }

    /// Map each scalar in this rectangle.
    pub fn map<U>(self, mut f: impl FnMut(T) -> U) -> Rect<U> {
        Rect::new(self.origin.map(&mut f), self.size.map(&mut f))
    }

    /// Returns a rectangle with a function called on its origin.
    pub fn map_origin(self, f: impl FnOnce(Point2<T>) -> Point2<T>) -> Self {
        Self::new(f(self.origin), self.size)
    }

    /// Returns a rectangle with a function called on its size.
    pub fn map_size(self, f: impl FnOnce(Size2<T>) -> Size2<T>) -> Self {
        Self::new(self.origin, f(self.size))
    }
}

impl<T: Zero> Zero for Rect<T> {
    const ZERO: Self = Self::new(Point2::ZERO, Size2::ZERO);
}

impl<T: Min + Max> Max for Rect<T> {
    /// A rectangle that encompasses the entire expressible coordinate space.
    const MAX: Self = Self::new(Point2::MIN, Size2::MAX);
}

impl From<Rect<f32>> for Path {
    fn from(rect: Rect<f32>) -> Self {
        let Rect { origin, size } = rect;

        let ne = origin + Vec2::new(size.w, 0.0);
        let se = ne + Vec2::new(0.0, size.h);
        let sw = se - Vec2::new(size.w, 0.0);

        Path::from_fn(|builder| {
            builder.open(origin);
            builder.line_to(ne);
            builder.line_to(se);
            builder.line_to(sw);
        })
    }
}

#[cfg(feature = "renderer")]
impl From<Rect<f64>> for vello::kurbo::Rect {
    fn from(rect: Rect<f64>) -> Self {
        use vello::kurbo;

        let Rect { origin: Point2 { x, y }, size: Size2 { w, h } } = rect;

        kurbo::Rect::from_origin_size((x, y), (w, h))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rect_contains() {
        let rect = Rect::from_size(Size2::splat(300.0));

        assert!(rect.contains_point(rect.origin));
        assert!(rect.contains_point(Point2::splat(100.0)));

        assert!(rect.contains(rect));
        assert!(rect.contains(Rect::new(Point2::splat(100.0), Size2::splat(100.0))));
    }
}
