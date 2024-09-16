use std::{fmt, mem};

pub use self::builder::*;
pub use self::iter::*;
use crate::math::Point2;

mod builder;
mod iter;
#[cfg(feature = "renderer")]
mod shape;

// TODO: make this wrap a `kurbo::BezPath`

/// A SVG-style vector path.
#[derive(Clone, PartialEq)]
pub struct Path {
    points: Vec<Point2<f32>>,
    verbs: Vec<Verb>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Verb {
    Open,
    LineTo,
    QuadTo,
    CubicTo,
    Close,
}

impl Path {
    /// Returns an empty path.
    pub const fn new() -> Self {
        Self { points: Vec::new(), verbs: Vec::new() }
    }

    /// Returns a [`PathBuilder`] for creating a path.
    pub const fn builder() -> PathBuilder {
        PathBuilder::new()
    }

    /// Creates a path in-place via a closure.
    pub fn from_fn(f: impl FnOnce(&mut PathBuilder)) -> Self {
        let mut builder = Self::builder();

        f(&mut builder);

        builder.build()
    }

    /// Returns the amount of elements in this path.
    pub fn len(&self) -> usize {
        self.verbs.len()
    }

    /// Returns `true` if this path contains no elements.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Returns an iterator of [`PathElement`]s.
    pub fn iter(&self) -> PathElements<'_> {
        PathElements { points: self.points.iter(), verbs: self.verbs.iter() }
    }

    /// Mutates this path in-place via a closure.
    pub fn update(&mut self, f: impl FnOnce(&mut PathBuilder)) {
        let points = mem::take(&mut self.points);
        let verbs = mem::take(&mut self.verbs);

        let mut builder = PathBuilder {
            points,
            verbs,
            move_required: true,
            last_point: (!self.points.is_empty()).then(|| self.points.len() - 1),
        };

        f(&mut builder);

        let Self { points, verbs } = builder.build();

        self.points = points;
        self.verbs = verbs;
    }

    /// Clears all elements from this path.
    pub fn clear(&mut self) {
        self.points.clear();
        self.verbs.clear();
    }
}

impl Default for Path {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Debug for Path {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut f = f.debug_list();

        for element in self.iter() {
            f.entry(&element);
        }

        f.finish()
    }
}

impl<'a> IntoIterator for &'a Path {
    type IntoIter = PathElements<'a>;
    type Item = PathElement;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_path_is_empty() {
        assert!(Path::new().is_empty());
    }

    #[test]
    fn single_open_becomes_empty_path() {
        let path = Path::from_fn(|builder| {
            builder.open(Point2::new(1.0, 1.0));
        });

        assert!(path.is_empty());
    }

    #[test]
    fn subpaths_are_closed_on_build() {
        let mut builder = Path::builder();

        builder.open(Point2::new(1.0, 0.0));
        builder.line_to(Point2::new(1.0, 1.0));
        builder.line_to(Point2::new(0.0, 1.0));
        builder.line_to(Point2::new(0.0, 0.0));

        let path = builder.build();
        let mut verbs = path.verbs.iter();

        assert_eq!(verbs.next(), Some(&Verb::Open));
        assert_eq!(verbs.next(), Some(&Verb::LineTo));
        assert_eq!(verbs.next(), Some(&Verb::LineTo));
        assert_eq!(verbs.next(), Some(&Verb::LineTo));
        assert_eq!(verbs.next(), Some(&Verb::Close));
        assert!(verbs.next().is_none());
    }

    #[test]
    fn empty_builder_becomes_empty_path() {
        let built = Path::from_fn(|_| {});
        let empty = Path::new();

        assert_eq!(built, empty);
    }

    #[test]
    fn path_is_same_after_update() {
        let mut path = Path::from_fn(|builder| {
            builder.line_to(Point2::new(1.0, 0.0));
            builder.line_to(Point2::new(1.0, 1.0));
            builder.line_to(Point2::new(0.0, 1.0));
        });
        let original = path.clone();

        path.update(|_| {});

        assert_eq!(path, original);
    }
}
