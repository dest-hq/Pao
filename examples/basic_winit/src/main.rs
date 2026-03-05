use std::sync::Arc;

use pao::{Canvas, Options, primit::Color};
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

struct State<'a> {
    window: Arc<Window>,
    size: winit::dpi::PhysicalSize<u32>,
    canvas: Canvas<'a>,
}

impl<'a> State<'a> {
    async fn new(window: Arc<Window>) -> State<'a> {
        let size = window.inner_size();
        State {
            window: window.clone(),
            size: size,
            canvas: Canvas::new(
                window.clone(),
                Options {
                    width: size.width,
                    height: size.height,
                    backends: Default::default(),
                    power_preference: pao::wgpu::PowerPreference::HighPerformance,
                    hints: pao::wgpu::MemoryHints::MemoryUsage,
                    mode: pao::wgpu::PresentMode::AutoVsync,
                },
            )
            .await
            .unwrap(),
        }
    }

    fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        self.size = new_size;

        // reconfigure the surface
        self.canvas.resize(self.size.width, self.size.height);
    }

    fn render(&mut self) {
        self.canvas.prepare(Color::rgb8(0, 255, 0));
        self.window.pre_present_notify();
        self.canvas.present();
    }
}

#[derive(Default)]
struct App<'a> {
    state: Option<State<'a>>,
}

impl<'a> ApplicationHandler for App<'a> {
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
