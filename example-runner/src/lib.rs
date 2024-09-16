use std::{mem, sync::Arc};

use keibrush::{
    wgpu::Surface, Affine2, RenderContext, RenderContextError, RenderDescriptor,
    Renderer, RendererDescriptor, Scene, Size2, Vec2,
};
use thiserror::Error;
pub use winit::error::EventLoopError;
pub use winit::window::WindowAttributes;
use winit::{
    application::ApplicationHandler,
    dpi::PhysicalSize,
    event::WindowEvent,
    event_loop::{ActiveEventLoop, EventLoop},
    window::{Window, WindowId},
};

/// Runs a simple [`winit`] application that draws with the provided closure.
pub fn run(
    window_attributes: WindowAttributes,
    f: impl FnMut(&mut Scene, Size2<f32>),
) -> Result<(), RunError> {
    let scene = Scene::new();

    let render_cx = RenderContext::new(Default::default())?;
    let window_state = WindowState::Uninit(window_attributes);
    let renderer = None;

    EventLoop::new()?
        .run_app(&mut Impl { f, scene, render_cx, window_state, renderer })
        .map_err(Into::into)
}

/// Error when calling [`run`].
#[derive(Debug, Error)]
pub enum RunError {
    #[error(transparent)]
    RenderContextError(#[from] RenderContextError),
    #[error(transparent)]
    EventLoopError(#[from] EventLoopError),
}

struct Impl<F> {
    f: F,
    scene: Scene,

    render_cx: RenderContext,
    window_state: WindowState,
    renderer: Option<Renderer>,
}

enum WindowState {
    /// [`ApplicationHandler::resumed`] hasn't been called yet.
    Uninit(WindowAttributes),
    /// The application has been suspended, the surface has been invalidated and needs to be recreated.
    Suspended(Arc<Window>),
    /// The window and its surface have been initialized.
    Init { window: Arc<Window>, surface: Surface<'static> },
}

impl<F> ApplicationHandler for Impl<F>
where
    F: FnMut(&mut Scene, Size2<f32>),
{
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let mut window_state = self.window_state.take();

        if let WindowState::Uninit(attributes) = window_state {
            let window = Arc::new(event_loop.create_window(attributes.clone()).unwrap());
            let surface = self.render_cx.instance.create_surface(window.clone()).unwrap();

            window_state = WindowState::Init { window, surface };
        }

        self.window_state = window_state;
    }

    fn suspended(&mut self, _event_loop: &ActiveEventLoop) {
        if let WindowState::Init { window, .. } = self.window_state.take() {
            self.window_state = WindowState::Suspended(window);
            // renderer will be recreated on [`WindowEvent::RedrawRequested`]
            self.renderer = None;
        }
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        window_id: WindowId,
        event: WindowEvent,
    ) {
        let mut window_state = self.window_state.take();

        if let WindowState::Suspended(window) = window_state {
            let surface = self.render_cx.instance.create_surface(window.clone()).unwrap();

            window_state = WindowState::Init { window, surface };
        }

        {
            let WindowState::Init { window, surface } = &window_state else {
                unreachable!();
            };

            if window.id() != window_id {
                return;
            }

            match event {
                WindowEvent::Resized(PhysicalSize { width, height }) => {
                    surface.configure(
                        &self.render_cx.device,
                        &surface
                            .get_default_config(&self.render_cx.adapter, width, height)
                            .unwrap(),
                    );
                    window.request_redraw();
                },
                WindowEvent::Destroyed => todo!(),
                WindowEvent::CloseRequested => event_loop.exit(),
                WindowEvent::RedrawRequested => {
                    let texture = surface.get_current_texture().unwrap();
                    let renderer = self.renderer.get_or_insert_with(|| {
                        Renderer::new(
                            &self.render_cx.device,
                            RendererDescriptor {
                                surface_format: Some(texture.texture.format()),
                                ..Default::default()
                            },
                        )
                        .unwrap()
                    });

                    let scale_factor: f32 = window.scale_factor() as _;
                    let physical_size =
                        Size2::new(texture.texture.width(), texture.texture.height());
                    let size = physical_size.map(|n| n as f32 / scale_factor);
                    let transform = Affine2::from_scale(Vec2::splat(scale_factor));

                    self.scene.clear();
                    (self.f)(&mut self.scene, size);

                    renderer
                        .render_to_surface(
                            &self.render_cx.device,
                            &self.render_cx.queue,
                            &texture,
                            &self.scene,
                            &RenderDescriptor {
                                global_transform: transform,
                                ..Default::default()
                            },
                        )
                        .unwrap();
                    texture.present();
                },
                _ => {},
            }
        }

        self.window_state = window_state;
    }
}

impl WindowState {
    fn take(&mut self) -> Self {
        // a little jank

        mem::replace(self, Self::Uninit(WindowAttributes::default()))
    }
}
