//! # 景brush
//!
//! `keibrush` Is an easy-to-use vector graphics library.
//!
//! The API revolves around [`Scene`], a sequence of vector graphics
//! [`Command`]s (fill, stroke, etc.). Scenes can be rendered with a
//! [`Renderer`] (if the `renderer` feature is enabled).

#![cfg_attr(doc, feature(doc_auto_cfg))]

use std::{slice, vec};

use element::{Brush, FillStyle, Layer, Path, Source, StrokeStyle, TextLayout, TextStyle};
use math::{Point2, Rect};
#[cfg(feature = "renderer")]
pub use vello::wgpu;

#[cfg(feature = "renderer")]
pub use self::renderer::*;

pub mod element;
pub mod math;
#[cfg(feature = "renderer")]
mod renderer;
mod util;

/// A vector scene.
#[derive(Debug, Clone)]
pub struct Scene {
    commands: Vec<Command>,
}

/// A single operation in a [`Scene`].
#[derive(Debug, Clone)]
pub enum Command {
    Fill {
        path: Path,
        brush: Brush,
        style: FillStyle,
    },
    Stroke {
        path: Path,
        brush: Brush,
        style: StrokeStyle,
    },
    DrawText {
        source: Source,
        bounds: Rect<f32>,
        style: TextStyle,
    },
    #[cfg(feature = "renderer")]
    DrawTextLayout {
        layout: TextLayout,
        origin: Point2<f32>,
    },
    /// Push a new layer onto the stack.
    PushLayer(Layer),
    /// Pop most recent layer off the stack.
    PopLayer,
}

impl Scene {
    /// Creates a new empty scene.
    pub const fn new() -> Self {
        let commands = Vec::new();

        Self { commands }
    }

    /// Returns the commands in this scene.
    pub fn commands(&self) -> &[Command] {
        &self.commands
    }

    /// Encodes a fill operation.
    pub fn fill(&mut self, path: impl Into<Path>, brush: impl Into<Brush>, style: FillStyle) {
        self.commands.push(Command::Fill { path: path.into(), brush: brush.into(), style });
    }

    /// Encodes a stroke operation.
    pub fn stroke(&mut self, path: impl Into<Path>, brush: impl Into<Brush>, style: StrokeStyle) {
        self.commands.push(Command::Stroke { path: path.into(), brush: brush.into(), style });
    }

    /// Encodes text.
    pub fn draw_text(&mut self, source: impl Into<Source>, bounds: Rect<f32>, style: TextStyle) {
        self.commands.push(Command::DrawText { source: source.into(), bounds, style });
    }

    /// Draws a [`TextLayout`].
    pub fn draw_text_layout(&mut self, layout: TextLayout, origin: Point2<f32>) {
        self.commands.push(Command::DrawTextLayout { layout, origin });
    }

    // TODO: allow manual pushing and popping of layers by validating each operation

    /// Encodes a new layer.
    ///
    /// All operations encoded in the closure will be within the layer.
    pub fn with_layer(&mut self, layer: Layer, f: impl FnOnce(&mut Self)) {
        self.commands.push(Command::PushLayer(layer));

        f(self);

        self.commands.push(Command::PopLayer);
    }

    /// Clears all commands from the scene.
    ///
    /// Does not change the global transform.
    pub fn clear(&mut self) {
        self.commands.clear();
    }
}

impl Default for Scene {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a> IntoIterator for &'a Scene {
    type IntoIter = slice::Iter<'a, Command>;
    type Item = &'a Command;

    fn into_iter(self) -> Self::IntoIter {
        self.commands.iter()
    }
}

impl IntoIterator for Scene {
    type IntoIter = vec::IntoIter<Command>;
    type Item = Command;

    fn into_iter(self) -> Self::IntoIter {
        self.commands.into_iter()
    }
}
