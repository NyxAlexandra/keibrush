use thiserror::Error;
use vello::wgpu::{Device, Queue, SurfaceTexture, Texture, TextureFormat, TextureViewDescriptor};
use vello::{kurbo, peniko};
pub use vello::{AaConfig, AaSupport};

use crate::element::{Color, FillStyle, Layer, TextContext, TextContextDescriptor, TextLayout};
use crate::math::{Affine2, Max, Rect, Size2};
use crate::{Command, Scene};

/// A renderer for a [`Scene`].
pub struct Renderer {
    inner: vello::Renderer,
    scratch: vello::Scene,
    output: vello::Scene,
    text_context: TextContext,
}

/// Parameters for creating a [`Renderer`].
pub struct RendererDescriptor {
    /// The texture format to use when calling [`Renderer::render_to_surface`].
    ///
    /// If `None`, the renderer cannot be used to render to surfaces.
    pub surface_format: Option<TextureFormat>,
    /// Anti-aliasing methods to support (default: [`AaSupport::all`]).
    pub antialiasing_support: AaSupport,
    /// Options for creating this renderer's [`TextContext`].
    pub text_context_desc: TextContextDescriptor,
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
}

impl Renderer {
    /// Creates a new renderer.
    pub fn new(device: &Device, desc: RendererDescriptor) -> Result<Self, RendererError> {
        let RendererDescriptor { surface_format, antialiasing_support, text_context_desc } = desc;

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
        let text_context = TextContext::new(text_context_desc);

        Ok(Self { inner, output, scratch, text_context })
    }

    /// Returns a reference to this renderer's [`TextContext`].
    pub fn text_context(&mut self) -> &mut TextContext {
        &mut self.text_context
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
                        // TODO: cache layouts
                        let mut layout = TextLayout::new();

                        // TODO: respect vertical bounds
                        layout.build(&mut self.text_context, source, style.clone());
                        layout.break_lines(bounds.size.w, style.alignment);
                        layout.render(bounds.origin, output);
                    },
                    Command::DrawTextLayout { layout, origin } => {
                        layout.render(*origin, output);
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
            text_context_desc: Default::default(),
        }
    }
}

impl Default for RenderDescriptor {
    fn default() -> Self {
        Self {
            antialiasing_method: AaConfig::Area,
            clear_color: Color::TRANSPARENT,
            global_transform: Affine2::IDENTITY,
        }
    }
}
