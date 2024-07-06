mod compute_pipeline;
mod context;
mod render_pipeline;

use crate::context::Context;
use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowAttributes,
};

// The lazy initialization of winit's ApplicationHandler is difficult to integrate with the
// lifetime of the Window object. Thus the old deprecated API is used in order to avoid this
// issue.
#[allow(deprecated)]
async fn run() {
    let event_loop = EventLoop::new().unwrap();
    event_loop.set_control_flow(ControlFlow::Poll);

    let window = event_loop
        .create_window(WindowAttributes::default())
        .unwrap();

    let mut graphics = Context::new(&window).await;

    let window_ref = &window;
    event_loop
        .run(move |event, elwt| {
            if let Event::WindowEvent {
                window_id: _,
                event,
            } = event
            {
                match event {
                    WindowEvent::Resized(new_size) => {
                        graphics.handle_resize(new_size);
                    }
                    WindowEvent::CloseRequested => {
                        elwt.exit();
                    }
                    WindowEvent::RedrawRequested => {
                        match graphics.render() {
                            Ok(_) => {}
                            Err(wgpu::SurfaceError::Lost) => {
                                graphics.reconfigure_surface();
                            }
                            Err(wgpu::SurfaceError::OutOfMemory) => elwt.exit(),
                            Err(e) => eprintln!("{:?}", e),
                        }
                        window_ref.request_redraw();
                    }
                    _ => {}
                }
            }
        })
        .unwrap();
}

fn main() {
    pollster::block_on(run());
}
