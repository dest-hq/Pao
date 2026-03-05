// Wgpu setup

use primit::{Circle, Color, Rect, RoundedRect};
use raw_window_handle::{HasDisplayHandle, HasWindowHandle};
use wgpu::{
    Backends, CompositeAlphaMode, Device, Instance, InstanceDescriptor, Limits, MemoryHints,
    PowerPreference, PresentMode, Queue, Surface, SurfaceConfiguration, TextureUsages,
};

use crate::commands::Commands;

// Options
#[derive(Default)]
pub struct Options {
    pub width: u32,
    pub height: u32,
    pub backends: Backends,
    pub power_preference: PowerPreference,
    pub hints: MemoryHints,
    pub mode: PresentMode,
}

pub struct Canvas<'a> {
    device: Device,
    queue: Queue,
    surface: Surface<'a>,
    config: SurfaceConfiguration,
    texture: Option<wgpu::SurfaceTexture>,
    commands: Vec<Commands>,
}

impl<'a> Canvas<'a> {
    pub async fn new<T>(window: T, options: Options) -> Result<Self, String>
    where
        T: HasWindowHandle + HasDisplayHandle + Send + Sync + 'a,
    {
        // Create instance
        let instance = Instance::new(&InstanceDescriptor {
            backends: options.backends,
            ..Default::default()
        });

        // Create surface
        let surface = instance
            .create_surface(window)
            .map_err(|_| "Can't create a surface")?;

        // Request adapter
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: options.power_preference,
                compatible_surface: Some(&surface),
                ..Default::default()
            })
            .await
            .map_err(|_| "Can't get adapter")?;

        // Request device
        let (device, queue) = adapter
            .request_device(&wgpu::DeviceDescriptor {
                required_limits: Limits::downlevel_webgl2_defaults()
                    .using_resolution(adapter.limits()),
                memory_hints: options.hints,
                ..Default::default()
            })
            .await
            .map_err(|_| "Can't get device")?;

        let format = surface.get_capabilities(&adapter).formats[0];

        // Configure surface
        let config = SurfaceConfiguration {
            usage: TextureUsages::RENDER_ATTACHMENT,
            format: format,
            width: options.width.max(1),
            height: options.height.max(1),
            present_mode: options.mode,
            alpha_mode: CompositeAlphaMode::Auto,
            view_formats: vec![format.add_srgb_suffix()],
            desired_maximum_frame_latency: 2,
        };

        surface.configure(&device, &config);

        Ok(Self {
            device,
            queue,
            surface,
            commands: Vec::new(),
            texture: None,
            config,
        })
    }

    pub fn fill_rect(&mut self, rect: Rect) {
        self.commands.push(Commands::RectCommand(rect));
    }

    pub fn fill_rounded_rect(&mut self, rounded_rect: RoundedRect) {
        self.commands
            .push(Commands::RoundedRectCommand(rounded_rect));
    }

    pub fn fill_circle(&mut self, circle: Circle) {
        self.commands.push(Commands::CircleCommand(circle));
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        // Configure surface
        self.config.width = width.max(1);
        self.config.height = height.max(1);
        self.surface.configure(&self.device, &self.config);
    }

    /// Prepare texture
    pub fn prepare(&mut self, background: Color) {
        if self.texture.is_some() {
            self.present();
        }

        // Create texture view
        let surface_texture = self.surface.get_current_texture().unwrap();
        let texture_view = surface_texture
            .texture
            .create_view(&wgpu::TextureViewDescriptor {
                format: Some(self.config.format.add_srgb_suffix()),
                ..Default::default()
            });

        let mut encoder = self.device.create_command_encoder(&Default::default());

        // Create the renderpass which will clear the screen.
        let renderpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: None,
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: &texture_view,
                depth_slice: None,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color {
                        r: background.r as f64 / 255.0,
                        g: background.g as f64 / 255.0,
                        b: background.b as f64 / 255.0,
                        a: background.a as f64 / 255.0,
                    }),
                    store: wgpu::StoreOp::Store,
                },
            })],
            depth_stencil_attachment: None,
            timestamp_writes: None,
            occlusion_query_set: None,
            multiview_mask: None,
        });

        // TODO: Draw commands

        // End the renderpass.
        drop(renderpass);

        // Submit the command in the queue to execute
        self.queue.submit([encoder.finish()]);
        self.texture = Some(surface_texture);
        self.commands.clear();
    }

    /// Display texture
    pub fn present(&mut self) {
        if let Some(texture) = self.texture.take() {
            texture.present();
        }
    }
}
