use pollster::FutureExt;
use thiserror::Error;
use vello::wgpu::{
    Adapter, Device, DeviceDescriptor, Instance, Queue, RequestAdapterOptions,
    RequestDeviceError,
};

/// A simple interface for creating a [`wgpu`](vello::wgpu) context and rendering scenes.
pub struct RenderContext {
    pub instance: Instance,
    pub adapter: Adapter,
    pub device: Device,
    pub queue: Queue,
}

/// Parameters for creating a [`RenderContext`].
#[derive(Default)]
pub struct RenderContextDescriptor<'a, 'b, 'c> {
    /// The [`Instance`] to use.
    pub instance: Instance,
    /// Options for creating the [`Adapter`].
    pub adapter: RequestAdapterOptions<'a, 'b>,
    /// Options for creating the [`Device`].
    pub device: DeviceDescriptor<'c>,
}

impl RenderContext {
    /// Creates a new render context.
    pub fn new(
        desc: RenderContextDescriptor<'_, '_, '_>,
    ) -> Result<Self, RenderContextError> {
        let RenderContextDescriptor { instance, adapter, device } = desc;

        let adapter = instance
            .request_adapter(&adapter)
            .block_on()
            .ok_or(RenderContextError::NoSuitableAdapter)?;
        let (device, queue) = adapter.request_device(&device, None).block_on()?;

        Ok(Self { instance, adapter, device, queue })
    }
}

/// Error when creating a [`RenderContext`].
#[derive(Debug, Error)]
pub enum RenderContextError {
    #[error("no suitable adapter found for the given configuration")]
    NoSuitableAdapter,
    #[error(transparent)]
    RequestDeviceError(#[from] RequestDeviceError),
}
