use primit::Color;
// use primit::{Circle, Rect, RoundedRect};
use raw_window_handle::{HasDisplayHandle, HasWindowHandle};
use wgpu::{
    Backends, CompositeAlphaMode, Device, Instance, InstanceDescriptor, Limits, MemoryHints,
    PowerPreference, PresentMode, Queue, Surface, SurfaceConfiguration, Texture, TextureFormat,
    TextureFormatFeatureFlags, TextureUsages, TextureView,
};

use crate::{Multisample, commands::Commands, features::RenderFeature};

/// Configuration options for creating a [`Canvas`]
#[derive(Default)]
pub struct Options {
    /// Window width in pixels
    pub width: u32,

    /// Window height in pixels
    pub height: u32,

    /// GPU backend preference (Vulkan, Metal, DX12, WebGPU)
    pub backends: Backends,

    /// Power preference for GPU selection
    pub power_preference: PowerPreference,

    /// Memory allocation hints for the GPU
    pub hints: MemoryHints,

    /// Present mode (VSync, Immediate, etc.)
    pub mode: PresentMode,

    /// Multisample anti-aliasing level (1x or 4x)
    pub multisample: Multisample,
}

/// A GPU-accelerated rendering canvas
pub struct Canvas {
    device: Device,
    queue: Queue,
    surface: Surface<'static>,
    config: SurfaceConfiguration,
    commands: Vec<Commands>,
    msaa_texture: Option<Texture>,
    msaa_view: Option<TextureView>,
    multisample_count: u32,
}

impl Canvas {
    /// Creates a new [`Canvas`] with specified options
    pub async fn new<T>(window: T, options: Options) -> Result<Self, String>
    where
        T: HasWindowHandle + HasDisplayHandle + Send + Sync + 'static,
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

        // Get capabilites of the surface
        let format = surface.get_capabilities(&adapter).formats[0];

        // Configure surface
        let config = SurfaceConfiguration {
            usage: TextureUsages::RENDER_ATTACHMENT,
            format: format,
            width: options.width.max(1),
            height: options.height.max(1),
            present_mode: options.mode,
            alpha_mode: CompositeAlphaMode::Auto,
            view_formats: vec![TextureFormat::Rgba8UnormSrgb],
            desired_maximum_frame_latency: 2,
        };

        surface.configure(&device, &config);

        // Get the supported texture format
        let sample_flags = adapter
            .get_texture_format_features(config.view_formats[0])
            .flags;

        let max_sample_count = {
            if options.multisample == Multisample::X4
                && sample_flags.contains(TextureFormatFeatureFlags::MULTISAMPLE_X4)
            {
                4
            } else {
                1
            }
        };

        let texture = if max_sample_count == 4 {
            // Creates a MSAA 4x texture
            Some(device.create_texture(&wgpu::TextureDescriptor {
                size: wgpu::Extent3d {
                    width: config.width,
                    height: config.height,
                    depth_or_array_layers: 1,
                },
                mip_level_count: 1,
                sample_count: 4,
                dimension: wgpu::TextureDimension::D2,
                format: config.view_formats[0],
                usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::TRANSIENT,
                label: None,
                view_formats: &[],
            }))
        } else {
            None
        };

        let texture_view = if let Some(texture) = &texture {
            Some(texture.create_view(&Default::default()))
        } else {
            None
        };

        Ok(Self {
            device,
            queue,
            surface,
            commands: Vec::new(),
            config,
            msaa_texture: texture,
            msaa_view: texture_view,
            multisample_count: max_sample_count,
        })
    }

    // pub fn draw_rect(&mut self, rect: Rect) {
    //     self.commands.push(Commands::RectCommand(rect));
    // }

    // pub fn draw_rounded_rect(&mut self, rounded_rect: RoundedRect) {
    //     self.commands
    //         .push(Commands::RoundedRectCommand(rounded_rect));
    // }

    // pub fn draw_circle(&mut self, circle: Circle) {
    //     self.commands.push(Commands::CircleCommand(circle));
    // }

    /// Returns the current MSAA (1 or 4)
    pub fn get_multisample_count(&self) -> &u32 {
        &self.multisample_count
    }

    /// Returns the reference to wgpu::Device
    pub fn get_device(&self) -> &Device {
        &self.device
    }

    /// Returns the reference to wgpu::SurfaceConfiguration
    pub fn get_surface_config(&self) -> &SurfaceConfiguration {
        &self.config
    }

    /// Returns the reference to wgpu::Queue
    pub fn get_queue(&self) -> &Queue {
        &self.queue
    }

    /// Resize the [`Canvas`]
    pub fn resize(&mut self, width: u32, height: u32) {
        // Reconfigure the surface
        self.config.width = width.max(1);
        self.config.height = height.max(1);
        self.surface.configure(&self.device, &self.config);

        // Recreating the MSAA texture with the new size
        self.msaa_texture = if self.multisample_count == 4 {
            Some(self.device.create_texture(&wgpu::TextureDescriptor {
                size: wgpu::Extent3d {
                    width: self.config.width,
                    height: self.config.height,
                    depth_or_array_layers: 1,
                },
                mip_level_count: 1,
                sample_count: 4,
                dimension: wgpu::TextureDimension::D2,
                format: self.config.view_formats[0],
                usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::TRANSIENT,
                label: None,
                view_formats: &[],
            }))
        } else {
            None
        };

        self.msaa_view = if let Some(texture) = &self.msaa_texture {
            Some(texture.create_view(&Default::default()))
        } else {
            None
        };
    }

    /// Adds a custom render feature to the [`Canvas`]
    pub fn draw_feature(&mut self, feature: Box<dyn RenderFeature>) {
        self.commands.push(Commands::FeatureCommand(feature));
    }

    /// Renders the current frame with the specified background color
    pub fn render(&mut self, background: Color) {
        // Get current texture
        let surface_texture = self
            .surface
            .get_current_texture()
            .expect("Failed to acquire next swap chain texture");

        // Create texture view
        let surface_view = surface_texture.texture.create_view(&Default::default());

        // Create encoder
        let mut encoder = self.device.create_command_encoder(&Default::default());

        let rpass_descriptor = if let Some(msaa_view) = &self.msaa_view {
            &wgpu::RenderPassDescriptor {
                label: None,
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: msaa_view,
                    depth_slice: None,
                    resolve_target: Some(&surface_view),
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: background.r as f64 / 255.0,
                            g: background.g as f64 / 255.0,
                            b: background.b as f64 / 255.0,
                            a: background.a as f64 / 100.0,
                        }),
                        store: wgpu::StoreOp::Discard,
                    },
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
                multiview_mask: None,
            }
        } else {
            &wgpu::RenderPassDescriptor {
                label: None,
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &surface_view,
                    depth_slice: None,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: background.r as f64 / 255.0,
                            g: background.g as f64 / 255.0,
                            b: background.b as f64 / 255.0,
                            a: background.a as f64 / 100.0,
                        }),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
                multiview_mask: None,
            }
        };

        let mut renderpass = encoder.begin_render_pass(rpass_descriptor);

        for cmd in &mut self.commands {
            match cmd {
                // Render the custom features
                Commands::FeatureCommand(feature) => {
                    feature.prepare(&self.device, &self.queue);
                    feature.render(&mut renderpass);
                } // _ => {}
            }
        }

        // End the renderpass.
        drop(renderpass);

        // Submit the command in the queue to execute
        self.queue.submit([encoder.finish()]);
        // Present the texture
        surface_texture.present();
        // Clear all commands
        self.commands.clear();
    }
}
