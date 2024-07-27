use std::sync::Arc;

use winit::dpi::PhysicalSize;
use winit::window::Window;

use crate::compute_pipeline::ComputePipeline;
use crate::debug::Debug;
use crate::render_pipeline::RenderPipeline;

pub struct Context {
    surface: wgpu::Surface<'static>,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    render_pipeline: Box<RenderPipeline>,
    compute_pipeline: Box<ComputePipeline>,
    debug: Debug,
    window: Arc<Window>,
}

impl Context {
    pub async fn new(window: Arc<Window>) -> Context {
        let window_size = window.inner_size();

        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::PRIMARY,
            ..Default::default()
        });

        let surface = instance.create_surface(window.clone()).unwrap();

        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::HighPerformance,
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .await
            .unwrap();

        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    required_features: wgpu::Features::empty(),
                    required_limits: wgpu::Limits::default(),
                    label: None,
                    memory_hints: wgpu::MemoryHints::default(),
                },
                None,
            )
            .await
            .unwrap();

        let surface_caps = surface.get_capabilities(&adapter);
        let surface_format = surface_caps
            .formats
            .iter()
            .find(|f| f.is_srgb())
            .copied()
            .unwrap_or(surface_caps.formats[0]);

        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: window_size.width,
            height: window_size.height,
            present_mode: surface_caps.present_modes[0],
            alpha_mode: surface_caps.alpha_modes[0],
            view_formats: vec![],
            desired_maximum_frame_latency: 2,
        };

        surface.configure(&device, &config);

        // TODO: see `[reconfigure_surface]`
        let compute_pipeline = Box::new(ComputePipeline::new(
            &device,
            wgpu::Extent3d {
                width: config.width,
                height: config.height,
                depth_or_array_layers: 1,
            },
        ));
        let render_pipeline = Box::new(RenderPipeline::new(
            &device,
            config.format,
            compute_pipeline.get_texture_view(),
        ));
        let debug = Debug::new(&device, &config, adapter.get_info());

        Self {
            surface,
            device,
            queue,
            config,
            render_pipeline,
            compute_pipeline,
            debug,
            window,
        }
    }

    pub fn reconfigure_surface(&mut self) {
        self.surface.configure(&self.device, &self.config);

        // TODO: This could be further optimized
        self.compute_pipeline = Box::new(ComputePipeline::new(
            &self.device,
            wgpu::Extent3d {
                width: self.config.width,
                height: self.config.height,
                depth_or_array_layers: 1,
            },
        ));
        self.render_pipeline = Box::new(RenderPipeline::new(
            &self.device,
            self.config.format,
            self.compute_pipeline.get_texture_view(),
        ));

        self.debug.handle_resize(
            self.config.width as f32,
            self.config.height as f32,
            &self.queue,
        );
    }

    pub fn handle_resize(&mut self, size: PhysicalSize<u32>) {
        self.config.width = size.width;
        self.config.height = size.height;
        self.reconfigure_surface();
    }

    pub fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        let frame = self.surface.get_current_texture()?;
        let view = frame
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });

        {
            let mut compute_pass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor {
                label: None,
                timestamp_writes: None,
            });

            self.compute_pipeline.compute(&mut compute_pass);
        }

        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: None,
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                ..Default::default()
            });

            self.render_pipeline.render(&mut render_pass);
            self.debug
                .render(&self.device, &self.queue, &mut render_pass);
        }

        self.queue.submit(std::iter::once(encoder.finish()));
        frame.present();

        Ok(())
    }

    pub fn window(&mut self) -> &mut Arc<Window> {
        &mut self.window
    }
}
