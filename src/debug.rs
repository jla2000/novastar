use std::time::{Duration, Instant};

use wgpu::SurfaceConfiguration;
use wgpu_text::{
    glyph_brush::{ab_glyph::FontRef, SectionBuilder, Text},
    BrushBuilder, TextBrush,
};

pub struct Debug {
    brush: TextBrush<FontRef<'static>>,
    fps: f32,
    last_render: Instant,
    frame_count: usize,
    backend: wgpu::Backend,
}

impl Debug {
    pub fn new(
        device: &wgpu::Device,
        config: &SurfaceConfiguration,
        adapter_info: wgpu::AdapterInfo,
    ) -> Self {
        let brush =
            BrushBuilder::using_font_bytes(include_bytes!("fonts/RobotoMonoNerdFont-Bold.ttf"))
                .unwrap()
                .build(device, config.width, config.height, config.format);

        Self {
            brush,
            fps: 0.0,
            last_render: Instant::now(),
            frame_count: 0,
            backend: adapter_info.backend,
        }
    }

    pub fn handle_resize(&mut self, width: f32, height: f32, queue: &wgpu::Queue) {
        self.brush.resize_view(width, height, queue);
    }

    pub fn render<'a>(
        &'a mut self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        render_pass: &mut wgpu::RenderPass<'a>,
    ) {
        self.frame_count += 1;

        let now = Instant::now();
        let duration = now - self.last_render;

        if duration >= Duration::from_millis(500) {
            self.fps = self.frame_count as f32 / duration.as_secs_f32();
            self.last_render = now;
            self.frame_count = 0;
        }

        let fps_string = format!("FPS: {}\nBackend: {:?}", self.fps as i32, self.backend);
        let section = SectionBuilder::default()
            .with_screen_position((10.0, 10.0))
            .add_text(
                Text::new(&fps_string)
                    .with_scale(26.0)
                    .with_color([0.0, 0.0, 0.0, 1.0]),
            );

        self.brush.queue(device, queue, [&section]).unwrap();
        self.brush.draw(render_pass);
    }
}
