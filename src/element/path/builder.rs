use super::{Path, Verb};
use crate::math::Point2;

/// An interface for creating and modifying [`Path`]s.
pub struct PathBuilder {
    pub(super) points: Vec<Point2<f32>>,
    pub(super) verbs: Vec<Verb>,
    pub(super) move_required: bool,
    pub(super) last_point: Option<usize>,
}

impl PathBuilder {
    /// Returns a new path builder.
    pub const fn new() -> Self {
        Self {
            points: Vec::new(),
            verbs: Vec::new(),
            move_required: true,
            last_point: None,
        }
    }

    /// Returns the amount of elements in this path.
    pub fn len(&self) -> usize {
        self.verbs.len()
    }

    /// Returns `true` if this path contains no elements.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Returns a path builder for a path.
    pub fn from_path(path: Path) -> Self {
        let Path { points, verbs, .. } = path;

        PathBuilder { points, verbs, move_required: true, last_point: None }
    }

    /// Opens a new subpath.
    pub fn open(&mut self, point: impl Into<Point2<f32>>) {
        self.move_required = false;
        self.last_point = Some(self.points.len());

        self.verbs.push(Verb::Open);
        self.points.push(point.into());
    }

    /// Adds a line to a point to this path.
    pub fn line_to(&mut self, point: impl Into<Point2<f32>>) {
        self.move_if_required();

        self.verbs.push(Verb::LineTo);
        self.points.push(point.into());
    }

    /// Adds a quadratic segment to a point to this path.
    pub fn quad_to(&mut self, p: impl Into<Point2<f32>>, c: impl Into<Point2<f32>>) {
        self.move_if_required();

        self.verbs.push(Verb::QuadTo);
        self.points.extend([p.into(), c.into()]);
    }

    /// Adds a cubic segment to a point to this path.
    pub fn cubic_to(
        &mut self,
        p: impl Into<Point2<f32>>,
        c1: impl Into<Point2<f32>>,
        c2: impl Into<Point2<f32>>,
    ) {
        self.move_if_required();

        self.verbs.push(Verb::CubicTo);
        self.points.extend([p.into(), c1.into(), c2.into()]);
    }

    /// Closes the current subpath.
    pub fn close(&mut self) {
        if !self.verbs.is_empty() {
            self.verbs.push(Verb::Close);
            self.move_required = true;
        }
    }

    /// Clears all verbs from the path.
    pub fn clear(&mut self) {
        self.points.clear();
        self.verbs.clear();

        self.move_required = true;
        self.last_point = None;
    }

    /// Builds a new [`Path`].
    pub fn build(self) -> Path {
        let Self { mut points, mut verbs, .. } = self;

        // if just a move, delete it
        if verbs.len() == 1 {
            points.clear();
            verbs.clear();
        }

        // if currently open, close
        if !self.move_required && verbs.last().is_some_and(|verb| verb != &Verb::Close) {
            verbs.push(Verb::Close);
        }

        Path { points, verbs }
    }

    fn move_if_required(&mut self) {
        if self.move_required {
            if let Some(idx) = self.last_point {
                self.open(self.points[idx]);
            } else {
                self.open(Point2::new(0.0, 0.0));
            }
        }
    }
}

impl Default for PathBuilder {
    fn default() -> Self {
        Self::new()
    }
}
