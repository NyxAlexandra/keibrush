use std::slice;

use super::Verb;
use crate::math::Point2;

/// The components of a [`Path`](super::Path).
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub enum PathElement {
    Open(Point2<f32>),
    LineTo(Point2<f32>),
    QuadTo { p: Point2<f32>, c: Point2<f32> },
    CubicTo { p: Point2<f32>, c1: Point2<f32>, c2: Point2<f32> },
    Close,
}

/// Iterator of [`PathElement`]s of a [`Path`](super::Path).
pub struct PathElements<'a> {
    pub(super) points: slice::Iter<'a, Point2<f32>>,
    pub(super) verbs: slice::Iter<'a, Verb>,
}

#[cfg(feature = "renderer")]
impl From<PathElement> for vello::kurbo::PathEl {
    fn from(path_element: PathElement) -> Self {
        use vello::kurbo;

        match path_element {
            PathElement::Open(Point2 { x, y }) => {
                kurbo::PathEl::MoveTo(kurbo::Point { x: x as _, y: y as _ })
            },
            PathElement::LineTo(Point2 { x, y }) => {
                kurbo::PathEl::LineTo(kurbo::Point { x: x as _, y: y as _ })
            },
            PathElement::QuadTo { p, c } => kurbo::PathEl::QuadTo(
                kurbo::Point { x: p.x as _, y: p.y as _ },
                kurbo::Point { x: c.x as _, y: c.y as _ },
            ),
            PathElement::CubicTo { p, c1, c2 } => kurbo::PathEl::CurveTo(
                kurbo::Point { x: p.x as _, y: p.y as _ },
                kurbo::Point { x: c1.x as _, y: c1.y as _ },
                kurbo::Point { x: c2.x as _, y: c2.y as _ },
            ),
            PathElement::Close => kurbo::PathEl::ClosePath,
        }
    }
}

impl Iterator for PathElements<'_> {
    type Item = PathElement;

    fn next(&mut self) -> Option<Self::Item> {
        // SAFETY: the internal invariant that verbs correspond to a certain amount of
        // points is guaranteed to be held
        let mut next = || unsafe { *self.points.next().unwrap_unchecked() };

        self.verbs.next().map(|verb| match verb {
            Verb::Open => PathElement::Open(next()),
            Verb::LineTo => PathElement::LineTo(next()),
            Verb::QuadTo => PathElement::QuadTo { p: next(), c: next() },
            Verb::CubicTo => PathElement::CubicTo { p: next(), c1: next(), c2: next() },
            Verb::Close => PathElement::Close,
        })
    }
}

impl ExactSizeIterator for PathElements<'_> {
    fn len(&self) -> usize {
        self.verbs.len()
    }
}
