use std::time::{Duration, Instant};

use wgpu::SurfaceConfiguration;
use wgpu_text::{
    glyph_brush::{ab_glyph::FontRef, SectionBuilder, Text},
    TextBrush,
};

pub struct Debug {
    brush: TextBrush<FontRef<'static>>,
    fps: f32,
    last_update: Instant,
    frame_count: usize,
}

impl Debug {
    pub fn new(device: &wgpu::Device, config: &SurfaceConfiguration) -> Self {
        let brush = wgpu_text::BrushBuilder::using_font_bytes(include_bytes!(
            "fonts/RobotoMonoNerdFont-Medium.ttf"
        ))
        .unwrap()
        .build(&device, config.width, config.height, config.format);

        Self {
            brush,
            fps: 0.0,
            last_update: Instant::now(),
            frame_count: 0,
        }
    }

    pub fn render<'a>(
        &'a mut self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        render_pass: &mut wgpu::RenderPass<'a>,
    ) {
        let now = Instant::now();
        self.frame_count += 1;
        let duration = now - self.last_update;

        if duration >= Duration::from_millis(500) {
            self.fps = self.frame_count as f32 / duration.as_secs_f32();
            self.last_update = now;
            self.frame_count = 0;
        }

        let fps_string = format!("FPS: {}", self.fps as i32);
        let section = SectionBuilder::default()
            .with_screen_position((10.0, 10.0))
            .add_text(
                Text::new(&fps_string)
                    .with_scale(26.0)
                    .with_color([1.0, 1.0, 1.0, 1.0]),
            );

        self.brush.queue(device, queue, [&section]).unwrap();
        self.brush.draw(render_pass);
    }
}
