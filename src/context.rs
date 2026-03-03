// Wgpu setup

use raw_window_handle::{HasDisplayHandle, HasWindowHandle};
use wgpu::{
    Adapter, Backends, CompositeAlphaMode, Device, Instance, InstanceDescriptor, PowerPreference,
    PresentMode, Queue, Surface, SurfaceConfiguration, TextureUsages,
};

// Options
#[derive(Default)]
pub struct Options {
    width: u32,
    height: u32,
    backends: Backends,
    power_preference: PowerPreference,
    vsync: bool,
}

pub struct RenderContext<'window> {
    pub device: Device,
    pub queue: Queue,
    pub surface: Surface<'window>,
    adapter: Adapter,
    options: Options,
}

impl<'window> RenderContext<'window> {
    pub async fn new<T>(window: T, options: Options) -> Result<Self, String>
    where
        T: HasWindowHandle + HasDisplayHandle + Clone + Send + Sync + 'window,
    {
        // Create instance
        let instance = Instance::new(&InstanceDescriptor {
            backends: options.backends,
            ..Default::default()
        });

        // Request adapter
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: options.power_preference,
                ..Default::default()
            })
            .await
            .map_err(|_| "Can't get adapter")?;

        // Request device
        let (device, queue) = adapter
            .request_device(&wgpu::DeviceDescriptor::default())
            .await
            .map_err(|_| "Can't get device")?;

        // Create adapter
        let surface = instance
            .create_surface(window)
            .map_err(|_| "Can't create a surface")?;

        let present_mode = match options.vsync {
            true => PresentMode::AutoVsync,
            false => PresentMode::AutoNoVsync,
        };

        // Configure surface
        let config = SurfaceConfiguration {
            usage: TextureUsages::RENDER_ATTACHMENT,
            format: surface.get_capabilities(&adapter).formats[0],
            width: options.width,
            height: options.height,
            present_mode: present_mode,
            alpha_mode: CompositeAlphaMode::Auto,
            view_formats: vec![],
            desired_maximum_frame_latency: 2,
        };

        surface.configure(&device, &config);

        Ok(Self {
            device,
            queue,
            surface,
            adapter,
            options,
        })
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        let present_mode = match self.options.vsync {
            true => PresentMode::AutoVsync,
            false => PresentMode::AutoNoVsync,
        };

        // Configure surface
        let config = SurfaceConfiguration {
            usage: TextureUsages::RENDER_ATTACHMENT,
            format: self.surface.get_capabilities(&self.adapter).formats[0],
            width: width,
            height: height,
            present_mode: present_mode,
            alpha_mode: CompositeAlphaMode::Auto,
            view_formats: vec![],
            desired_maximum_frame_latency: 2,
        };

        self.surface.configure(&self.device, &config);
    }
}
