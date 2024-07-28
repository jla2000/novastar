mod compute_pipeline;
mod context;
mod debug;
mod render_pipeline;

use std::sync::Arc;

use crate::context::Context;
use winit::{
    application::ApplicationHandler,
    dpi::PhysicalSize,
    event::WindowEvent,
    event_loop::{ControlFlow, EventLoop},
    window::Window,
};

struct App {
    context: Option<Context>,
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        if self.context.is_some() {
            return;
        }

        let window_attrs = Window::default_attributes()
            .with_inner_size(PhysicalSize::new(1900, 1205))
            .with_resizable(false)
            .with_title("novastar");

        let window = Arc::new(event_loop.create_window(window_attrs).unwrap());
        self.context = Some(pollster::block_on(Context::new(window)));
    }

    fn window_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        _window_id: winit::window::WindowId,
        event: WindowEvent,
    ) {
        if let Some(context) = &mut self.context {
            match event {
                WindowEvent::Resized(new_size) => {
                    context.handle_resize(new_size);
                }
                WindowEvent::CloseRequested => {
                    event_loop.exit();
                }
                WindowEvent::RedrawRequested => {
                    match context.render() {
                        Ok(_) => {}
                        Err(wgpu::SurfaceError::Lost) => {
                            context.reconfigure_surface();
                        }
                        Err(wgpu::SurfaceError::OutOfMemory) => event_loop.exit(),
                        Err(e) => eprintln!("{:?}", e),
                    }
                    context.window().request_redraw();
                }
                _ => {}
            }
        }
    }
}

fn main() {
    env_logger::init();

    let event_loop = EventLoop::new().unwrap();
    event_loop.set_control_flow(ControlFlow::Poll);
    _ = event_loop.run_app(&mut App { context: None });
}
