use std::sync::Arc;

use pao::{Canvas, Options, primit::Color, wgpu::Limits};
use winit::{
    application::ApplicationHandler,
    event::WindowEvent,
    event_loop::{ActiveEventLoop, EventLoop},
    window::{Window, WindowId},
};

fn main() {
    let event_loop = EventLoop::new().unwrap();

    event_loop.run_app(&mut App::default()).unwrap();
}

struct State {
    window: Arc<Window>,
    canvas: Canvas,
}

impl State {
    async fn new(window: Arc<Window>) -> State {
        let size = window.inner_size();
        State {
            window: window.clone(),
            canvas: Canvas::new(
                window,
                Options {
                    width: size.width,
                    height: size.height,
                    backends: Default::default(),
                    power_preference: pao::wgpu::PowerPreference::HighPerformance,
                    hints: pao::wgpu::MemoryHints::MemoryUsage,
                    mode: pao::wgpu::PresentMode::AutoVsync,
                    limits: Limits::downlevel_webgl2_defaults(),
                },
            )
            .await
            .unwrap(),
        }
    }

    fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        self.canvas.resize(new_size.width, new_size.height);
    }

    fn render(&mut self) {
        self.window.pre_present_notify();
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
