use std::{fmt, mem};

use parley::style::{FontStack, StyleProperty};
use parley::Layout;
use vello::glyph::skrifa::prelude::NormalizedCoord;
use vello::peniko;

use super::{Brush, SourceRef, TextAlignment, TextContext, TextStyle};
use crate::math::{Affine2, Point2, Size2};

/// Precalculated layout of some text.
#[derive(Clone)]
pub struct TextLayout {
    inner: Layout<peniko::Brush>,
}

impl TextLayout {
    /// Creates an empty layout.
    pub fn new() -> Self {
        let inner = Layout::new();

        Self { inner }
    }

    /// Returns the size of this layout.
    pub fn size(&self) -> Size2<f32> {
        Size2::new(self.inner.width(), self.inner.height())
    }

    /// Breaks all lines in this text layout to fit within a certain width.
    pub fn break_lines(&mut self, width: f32, alignment: TextAlignment) {
        self.inner.break_all_lines(Some(width), alignment.into());
    }

    /// Builds this layout with source and style.
    pub fn build<'a>(
        &mut self,
        text_cx: &mut TextContext,
        source: impl Into<SourceRef<'a>>,
        style: TextStyle,
    ) {
        let source = source.into();
        let text = source.text();

        let brush: peniko::Brush = Brush::Solid(style.color).into();
        let size = style.size;

        let font_family: parley::style::FontFamily = (&style.font.family).into();
        let font_weight: parley::style::FontWeight = style.font.weight.into();
        let font_style: parley::style::FontStyle = style.font.style.into();

        let mut builder = text_cx.layout_cx.ranged_builder(&mut text_cx.font_cx, &text, 1.0);

        let mut font_stack: Vec<parley::style::FontFamily> = Vec::new();

        font_stack.push(font_family.into());
        font_stack.extend(style.font.fallback.iter().map(parley::style::FontFamily::from));

        builder.push_default(&StyleProperty::FontStack(FontStack::List(&font_stack)));
        builder.push_default(&StyleProperty::FontSize(size));
        builder.push_default(&StyleProperty::FontWeight(font_weight));
        builder.push_default(&StyleProperty::FontStyle(font_style));
        builder.push_default(&StyleProperty::Brush(brush));

        if let SourceRef::Rich(spans) = source {
            let mut start = 0;

            for span in spans {
                let range = start..(start + span.source.len());

                if let Some(font_family) = span.font_family.as_ref() {
                    let prev = mem::replace(&mut font_stack[0], font_family.into());

                    builder.push(
                        &StyleProperty::FontStack(FontStack::List(&font_stack)),
                        range.clone(),
                    );

                    font_stack[0] = prev;
                }
                if let Some(font_style) = span.font_style {
                    builder.push(&StyleProperty::FontStyle(font_style.into()), range.clone());
                }
                if let Some(font_weight) = span.font_weight {
                    builder.push(&StyleProperty::FontWeight(font_weight.into()), range.clone());
                }
                if let Some(color) = span.color {
                    let brush: Brush = color.into();

                    builder.push(&StyleProperty::Brush(brush.into()), range.clone());
                }
                if let Some(size) = span.size {
                    builder.push(&StyleProperty::FontSize(size), range);
                }

                start += span.source.len();
            }
        }

        builder.build_into(&mut self.inner);
        self.break_lines(f32::MAX, style.alignment);
    }

    pub(crate) fn render(&self, origin: Point2<f32>, output: &mut vello::Scene) {
        for line in self.inner.lines() {
            for glyph_run in line.glyph_runs() {
                let run = glyph_run.run();

                let mut run_x = glyph_run.offset();
                let run_y = glyph_run.baseline();

                let coords: Vec<_> = run
                    .normalized_coords()
                    .iter()
                    .copied()
                    .map(NormalizedCoord::from_bits)
                    .collect();

                output
                    .draw_glyphs(run.font())
                    .brush(&glyph_run.style().brush)
                    .transform(Affine2::from_translation(origin.to_vec()).into())
                    .font_size(run.font_size())
                    .normalized_coords(&coords)
                    .draw(
                        peniko::Fill::NonZero,
                        glyph_run.glyphs().map(
                            |parley::layout::Glyph { id, x, y, advance, .. }| {
                                let out =
                                    vello::glyph::Glyph { id: id as _, x: x + run_x, y: y + run_y };

                                run_x += advance;

                                out
                            },
                        ),
                    );
            }
        }
    }
}

impl fmt::Debug for TextLayout {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("TextLayout").field("size", &self.size()).finish_non_exhaustive()
    }
}
