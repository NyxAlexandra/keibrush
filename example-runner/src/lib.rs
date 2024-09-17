use std::mem;
use std::sync::Arc;

pub extern crate winit;

use keibrush::math::{Affine2, Size2, Vec2};
use keibrush::wgpu::{Adapter, Device, Instance, Queue, RequestDeviceError, Surface};
use keibrush::{RenderDescriptor, Renderer, RendererDescriptor, Scene};
use pollster::FutureExt;
use thiserror::Error;
use winit::application::ApplicationHandler;
use winit::dpi::PhysicalSize;
use winit::error::EventLoopError;
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, EventLoop};
use winit::window::{Window, WindowAttributes, WindowId};

/// Runs a simple [`winit`] application that draws with the provided closure.
pub fn run(desc: ExampleDescriptor, f: impl FnMut(&mut Scene, Size2<f32>)) -> Result<(), RunError> {
    let ExampleDescriptor { window_attributes, respect_scale_factor, always_redraw } = desc;

    let scene = Scene::new();

    let instance = Instance::default();
    let adapter = instance
        .request_adapter(&Default::default())
        .block_on()
        .ok_or(RunError::NoSuitableAdapter)?;
    let (device, queue) = adapter.request_device(&Default::default(), None).block_on()?;

    let window_state = WindowState::Uninit(window_attributes);
    let renderer = None;

    EventLoop::new()?
        .run_app(&mut Impl {
            f,
            scene,
            always_redraw,
            instance,
            adapter,
            device,
            queue,
            window_state,
            renderer,
            respect_scale_factor,
        })
        .map_err(Into::into)
}

/// Configuration for an example.
#[derive(Default)]
pub struct ExampleDescriptor {
    /// The attributes used to create the window.
    pub window_attributes: WindowAttributes,
    /// Whether to respected the window's requested scale factor.
    pub respect_scale_factor: bool,
    /// Whether to always redraw.
    ///
    /// Useful if an application is animated.
    pub always_redraw: bool,
}

/// Error when calling [`run`].
#[derive(Debug, Error)]
pub enum RunError {
    #[error("no suitable adapter found for the given configuration")]
    NoSuitableAdapter,
    #[error(transparent)]
    RequestDeviceError(#[from] RequestDeviceError),
    #[error(transparent)]
    EventLoopError(#[from] EventLoopError),
}

struct Impl<F> {
    f: F,
    scene: Scene,
    always_redraw: bool,

    instance: Instance,
    adapter: Adapter,
    device: Device,
    queue: Queue,

    window_state: WindowState,
    renderer: Option<Renderer>,

    respect_scale_factor: bool,
}

enum WindowState {
    /// [`ApplicationHandler::resumed`] hasn't been called yet.
    Uninit(WindowAttributes),
    /// The application has been suspended, the surface has been invalidated and
    /// needs to be recreated.
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
            let surface = self.instance.create_surface(window.clone()).unwrap();

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
            let surface = self.instance.create_surface(window.clone()).unwrap();

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
                        &self.device,
                        &surface.get_default_config(&self.adapter, width, height).unwrap(),
                    );
                    window.request_redraw();
                },
                WindowEvent::Destroyed => todo!(),
                WindowEvent::CloseRequested => event_loop.exit(),
                WindowEvent::RedrawRequested => {
                    let texture = surface.get_current_texture().unwrap();
                    let renderer = self.renderer.get_or_insert_with(|| {
                        Renderer::new(
                            &self.device,
                            RendererDescriptor {
                                surface_format: Some(texture.texture.format()),
                                ..Default::default()
                            },
                        )
                        .unwrap()
                    });

                    let scale_factor: f32 =
                        if self.respect_scale_factor { window.scale_factor() as _ } else { 1.0 };
                    let physical_size =
                        Size2::new(texture.texture.width(), texture.texture.height());
                    let size = physical_size.map(|n| n as f32 / scale_factor);
                    let transform = Affine2::from_scale(Vec2::splat(scale_factor));

                    self.scene.clear();
                    (self.f)(&mut self.scene, size);

                    renderer
                        .render_to_surface(
                            &self.device,
                            &self.queue,
                            &texture,
                            &self.scene,
                            &RenderDescriptor { global_transform: transform, ..Default::default() },
                        )
                        .unwrap();
                    texture.present();
                },
                _ => {},
            }

            if self.always_redraw {
                window.request_redraw();
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
