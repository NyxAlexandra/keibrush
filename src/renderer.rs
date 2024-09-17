use std::mem;

use parley::fontique::{Collection, CollectionOptions};
use parley::style::{FontStack, StyleProperty};
use parley::{FontContext, LayoutContext};
use thiserror::Error;
use vello::glyph::skrifa::instance::NormalizedCoord;
use vello::wgpu::{Device, Queue, SurfaceTexture, Texture, TextureFormat, TextureViewDescriptor};
use vello::{kurbo, peniko};
pub use vello::{AaConfig, AaSupport};

use crate::element::{Brush, Color, FillStyle, Layer, Source};
use crate::math::{Affine2, Max, Rect, Size2};
use crate::{Command, Scene};

/// A renderer for a [`Scene`].
pub struct Renderer {
    inner: vello::Renderer,
    scratch: vello::Scene,
    output: vello::Scene,
    font_cx: FontContext,
    layout_cx: LayoutContext<peniko::Brush>,
}

/// Parameters for creating a [`Renderer`].
pub struct RendererDescriptor {
    /// The texture format to use when calling [`Renderer::render_to_surface`].
    ///
    /// If `None`, the renderer cannot be used to render to surfaces.
    pub surface_format: Option<TextureFormat>,
    /// Anti-aliasing methods to support (default: [`AaSupport::all`]).
    pub antialiasing_support: AaSupport,
    /// Whether to load fonts from the system (default: `true`).
    pub use_system_fonts: bool,
}

/// Parameters for calling [`Renderer::render_to_texture`] and
/// [`Renderer::render_to_surface`].
#[derive(Clone, Copy)]
pub struct RenderDescriptor {
    /// The method of anti-aliasing to use.
    pub antialiasing_method: AaConfig,
    /// The base color.
    pub clear_color: Color,
    /// Transform applied to the entire scene.
    pub global_transform: Affine2<f32>,
    /// Transform applied to text.
    pub text_transform: Affine2<f32>,
}

impl Renderer {
    /// Creates a new renderer.
    pub fn new(device: &Device, desc: RendererDescriptor) -> Result<Self, RendererError> {
        let RendererDescriptor { surface_format, antialiasing_support, use_system_fonts } = desc;

        let inner = vello::Renderer::new(
            device,
            vello::RendererOptions {
                surface_format,
                use_cpu: false,
                antialiasing_support,
                num_init_threads: None,
            },
        )?;
        let output = vello::Scene::new();
        let scratch = vello::Scene::new();
        let font_cx = FontContext {
            collection: Collection::new(CollectionOptions {
                system_fonts: use_system_fonts,
                ..Default::default()
            }),
            ..Default::default()
        };
        let layout_cx = LayoutContext::new();

        Ok(Self { inner, output, scratch, font_cx, layout_cx })
    }

    fn prepare(&mut self, scene: &Scene, desc: &RenderDescriptor) {
        let needs_final_transform = desc.global_transform != Affine2::IDENTITY;

        self.output.reset();
        self.scratch.reset();

        {
            let output = if !needs_final_transform { &mut self.output } else { &mut self.scratch };

            for command in scene {
                match command {
                    Command::Fill { path, brush, style } => {
                        let FillStyle { rule } = *style;

                        let style: peniko::Fill = rule.into();
                        let brush: peniko::Brush = brush.clone().into();

                        output.fill(style, kurbo::Affine::IDENTITY, &brush, None, path);
                    },
                    Command::Stroke { path, brush, style } => {
                        let stroke: kurbo::Stroke = (*style).into();
                        let brush: peniko::Brush = brush.clone().into();

                        output.stroke(&stroke, kurbo::Affine::IDENTITY, &brush, None, path);
                    },
                    Command::DrawText { source, bounds, style } => {
                        let text = source.text();

                        let brush: peniko::Brush = Brush::Solid(style.color).into();
                        let size = style.size;
                        let alignment: parley::layout::Alignment = style.alignment.into();

                        let font_family: parley::style::FontFamily = (&style.font.family).into();
                        let font_weight: parley::style::FontWeight = style.font.weight.into();
                        let font_style: parley::style::FontStyle = style.font.style.into();

                        let mut builder =
                            self.layout_cx.ranged_builder(&mut self.font_cx, &text, 1.0);

                        let mut font_stack: Vec<parley::style::FontFamily> = Vec::new();

                        font_stack.push(font_family.into());
                        font_stack.extend(
                            style.font.fallback.iter().map(parley::style::FontFamily::from),
                        );

                        builder
                            .push_default(&StyleProperty::FontStack(FontStack::List(&font_stack)));
                        builder.push_default(&StyleProperty::FontSize(size));
                        builder.push_default(&StyleProperty::FontWeight(font_weight));
                        builder.push_default(&StyleProperty::FontStyle(font_style));
                        builder.push_default(&StyleProperty::Brush(brush));

                        if let Source::Rich(spans) = source {
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
                                    builder.push(
                                        &StyleProperty::FontStyle(font_style.into()),
                                        range.clone(),
                                    );
                                }
                                if let Some(font_weight) = span.font_weight {
                                    builder.push(
                                        &StyleProperty::FontWeight(font_weight.into()),
                                        range.clone(),
                                    );
                                }
                                if let Some(color) = span.color {
                                    let brush: Brush = color.into();

                                    builder
                                        .push(&StyleProperty::Brush(brush.into()), range.clone());
                                }
                                if let Some(size) = span.size {
                                    builder.push(&StyleProperty::FontSize(size), range);
                                }

                                start += span.source.len();
                            }
                        }

                        let mut layout = builder.build();

                        // TODO: respect vertical bounds
                        layout.break_all_lines(Some(bounds.size.w), alignment);

                        for line in layout.lines() {
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
                                    .transform(
                                        desc.text_transform
                                            .map_translation(|translation| {
                                                translation + bounds.origin.to_vec()
                                            })
                                            .into(),
                                    )
                                    .font_size(run.font_size())
                                    .normalized_coords(&coords)
                                    .draw(
                                        peniko::Fill::NonZero,
                                        glyph_run.glyphs().map(
                                            |parley::layout::Glyph {
                                                 id, x, y, advance, ..
                                             }| {
                                                let out = vello::glyph::Glyph {
                                                    id: id as _,
                                                    x: x + run_x,
                                                    y: y + run_y,
                                                };

                                                run_x += advance;

                                                out
                                            },
                                        ),
                                    );
                            }
                        }
                    },
                    Command::PushLayer(layer) => {
                        let Layer { transform, blend_mode, clip, alpha } = layer;

                        let transform: kurbo::Affine = (*transform).into();

                        if let Some(clip) = clip {
                            output.push_layer(*blend_mode, *alpha, transform, clip);
                        } else {
                            let clip: kurbo::Rect = Rect::from_size(Size2::MAX).into();

                            output.push_layer(*blend_mode, *alpha, transform, &clip);
                        }
                    },
                    Command::PopLayer => output.pop_layer(),
                }
            }
        }

        if needs_final_transform {
            self.output.append(&self.scratch, Some(desc.global_transform.into()));
        }
    }

    /// Render to a texture.
    pub fn render_to_texture(
        &mut self,
        device: &Device,
        queue: &Queue,
        texture: &Texture,
        scene: &Scene,
        desc: &RenderDescriptor,
    ) -> Result<(), RendererError> {
        self.prepare(scene, desc);

        let RenderDescriptor { antialiasing_method, clear_color, .. } = *desc;

        let view = texture.create_view(&TextureViewDescriptor::default());

        self.inner.render_to_texture(
            device,
            queue,
            &self.output,
            &view,
            &vello::RenderParams {
                base_color: clear_color.into(),
                width: texture.width(),
                height: texture.height(),
                antialiasing_method,
            },
        )?;

        Ok(())
    }

    /// Render to a surface.
    pub fn render_to_surface(
        &mut self,
        device: &Device,
        queue: &Queue,
        surface: &SurfaceTexture,
        scene: &Scene,
        desc: &RenderDescriptor,
    ) -> Result<(), RendererError> {
        self.prepare(scene, desc);

        let RenderDescriptor { antialiasing_method, clear_color, .. } = *desc;

        self.inner.render_to_surface(
            device,
            queue,
            &self.output,
            surface,
            &vello::RenderParams {
                base_color: clear_color.into(),
                width: surface.texture.width(),
                height: surface.texture.height(),
                antialiasing_method,
            },
        )?;

        Ok(())
    }
}

/// Error when creating a [`Renderer`] and when calling
/// [`Renderer::render_to_texture`] or [`Renderer::render_to_surface`].
#[derive(Debug, Error)]
pub enum RendererError {
    #[error(transparent)]
    Inner(#[from] vello::Error),
}

impl Default for RendererDescriptor {
    fn default() -> Self {
        Self {
            surface_format: None,
            antialiasing_support: AaSupport::all(),
            use_system_fonts: true,
        }
    }
}

impl Default for RenderDescriptor {
    fn default() -> Self {
        Self {
            antialiasing_method: AaConfig::Area,
            clear_color: Color::TRANSPARENT,
            global_transform: Affine2::IDENTITY,
            text_transform: Affine2::IDENTITY,
        }
    }
}
