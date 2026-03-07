use std::sync::Arc;

use pao::{Canvas, Options, features::RenderFeature, primit::Color};
use winit::{
    application::ApplicationHandler,
    event::WindowEvent,
    event_loop::{ActiveEventLoop, EventLoop},
    window::{Window, WindowId},
};

#[derive(Clone)]
pub struct TriangleFeature {
    pipeline: pao::wgpu::RenderPipeline,
}

impl TriangleFeature {
    pub fn new(
        device: &pao::wgpu::Device,
        config: &pao::wgpu::SurfaceConfiguration,
        multisample_count: u32,
    ) -> Self {
        // load shader
        let shader = device.create_shader_module(pao::wgpu::ShaderModuleDescriptor {
            label: Some("Triangle Shader"),
            source: pao::wgpu::ShaderSource::Wgsl(include_str!("shader.wgsl").into()),
        });

        // layout pipeline
        let layout = device.create_pipeline_layout(&pao::wgpu::PipelineLayoutDescriptor {
            label: Some("Triangle Pipeline Layout"),
            bind_group_layouts: &[],
            immediate_size: Default::default(),
        });

        // create pipeline
        let pipeline = device.create_render_pipeline(&pao::wgpu::RenderPipelineDescriptor {
            multiview_mask: None,
            label: Some("Triangle Pipeline"),
            layout: Some(&layout),
            vertex: pao::wgpu::VertexState {
                compilation_options: Default::default(),
                module: &shader,
                entry_point: Some("vs_main"),
                buffers: &[],
            },
            fragment: Some(pao::wgpu::FragmentState {
                compilation_options: Default::default(),
                module: &shader,
                entry_point: Some("fs_main"),
                targets: &[Some(pao::wgpu::ColorTargetState {
                    format: config.format,
                    blend: Some(pao::wgpu::BlendState::REPLACE),
                    write_mask: pao::wgpu::ColorWrites::ALL,
                })],
            }),
            primitive: pao::wgpu::PrimitiveState::default(),
            depth_stencil: None,
            multisample: pao::wgpu::MultisampleState {
                count: multisample_count,
                ..Default::default()
            },
            cache: None,
        });

        Self { pipeline }
    }
}

impl RenderFeature for TriangleFeature {
    fn prepare(&mut self, _device: &pao::wgpu::Device, _queue: &pao::wgpu::Queue) {}

    fn render(&mut self, pass: &mut pao::wgpu::RenderPass) {
        pass.set_pipeline(&self.pipeline);
        pass.draw(0..3, 0..1);
    }
}

fn main() {
    let event_loop = EventLoop::new().unwrap();

    event_loop.run_app(&mut App::default()).unwrap();
}

struct State {
    window: Arc<Window>,
    triangle_feature: Box<TriangleFeature>,
    canvas: Canvas,
}

impl State {
    async fn new(window: Arc<Window>) -> State {
        let size = window.inner_size();

        let canvas = Canvas::new(
            window.clone(),
            Options {
                width: size.width,
                height: size.height,
                backends: Default::default(),
                power_preference: pao::wgpu::PowerPreference::HighPerformance,
                hints: pao::wgpu::MemoryHints::MemoryUsage,
                mode: pao::wgpu::PresentMode::AutoVsync,
                multisample: pao::Multisample::X4,
            },
        )
        .await
        .unwrap();

        State {
            window: window,
            triangle_feature: Box::new(TriangleFeature::new(
                canvas.get_device(),
                &canvas.get_surface_config(),
                canvas.get_multisample_count().clone(),
            )),
            canvas,
        }
    }

    fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        self.canvas.resize(new_size.width, new_size.height);
    }

    fn render(&mut self) {
        self.window.pre_present_notify();
        self.canvas.draw_feature(self.triangle_feature.clone());
        self.canvas.render(Color::rgb8(0, 255, 0));
    }
}

#[derive(Default)]
struct App {
    state: Option<State>,
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        // Create window object
        let window = Arc::new(
            event_loop
                .create_window(Window::default_attributes())
                .unwrap(),
        );

        let state = pollster::block_on(State::new(window.clone()));
        self.state = Some(state);

        window.request_redraw();
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {
        let state = self.state.as_mut().unwrap();
        match event {
            WindowEvent::CloseRequested => {
                event_loop.exit();
            }
            WindowEvent::RedrawRequested => {
                state.render();
            }
            WindowEvent::Resized(size) => {
                state.resize(size);
            }
            _ => (),
        }
    }
}
