use std::time::{Duration, Instant};

use wgpu::SurfaceConfiguration;
use wgpu_text::{
    glyph_brush::{
        ab_glyph::FontRef, BuiltInLineBreaker, HorizontalAlign, Layout, SectionBuilder, Text,
        VerticalAlign,
    },
    BrushBuilder, TextBrush,
};

pub struct Debug {
    brush: TextBrush<FontRef<'static>>,
    fps: f32,
    last_update_time: Instant,
    frames_since_last_update: usize,
    backend: wgpu::Backend,
    size: (u32, u32),
}

const COMMIT_HASH: &str = include_str!(concat!(env!("OUT_DIR"), "/commit-hash.txt"));

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
            last_update_time: Instant::now(),
            frames_since_last_update: 0,
            backend: adapter_info.backend,
            size: (config.width, config.height),
        }
    }

    pub fn handle_resize(&mut self, width: u32, height: u32, queue: &wgpu::Queue) {
        self.brush.resize_view(width as f32, height as f32, queue);
        self.size = (width, height);
    }

    pub fn render<'a>(
        &'a mut self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        render_pass: &mut wgpu::RenderPass<'a>,
    ) {
        self.frames_since_last_update += 1;

        let now = Instant::now();
        let time_since_last_update = now - self.last_update_time;

        if time_since_last_update >= Duration::from_millis(500) {
            self.fps = self.frames_since_last_update as f32 / time_since_last_update.as_secs_f32();
            self.last_update_time = now;
            self.frames_since_last_update = 0;
        }

        let fps_string = format!(
            "FPS: {}\nBackend: {:?}\nSurface: {}x{}",
            self.fps as i32, self.backend, self.size.0, self.size.1
        );
        let info_section = SectionBuilder::default()
            .with_screen_position((10.0, 10.0))
            .add_text(Text::new(&fps_string).with_scale(26.0));

        let commit_section = SectionBuilder::default()
            .with_screen_position((10.0, self.size.1 as f32 - 30.0))
            .add_text(Text::new(COMMIT_HASH).with_scale(26.0));

        self.brush
            .queue(device, queue, [&info_section, &commit_section])
            .unwrap();
        self.brush.draw(render_pass);
    }
}
