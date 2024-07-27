use wgpu::PipelineCompilationOptions;

pub struct ComputePipeline {
    pipeline: wgpu::ComputePipeline,
    texture: wgpu::Texture,
    texture_view: wgpu::TextureView,
}

impl ComputePipeline {
    pub fn new(
        device: &wgpu::Device,
        output_format: wgpu::TextureFormat,
        output_size: wgpu::Extent3d,
    ) -> Self {
        let shader = device.create_shader_module(wgpu::include_wgsl!("compute.wgsl"));

        let compute_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: None,
                bind_group_layouts: &[],
                push_constant_ranges: &[],
            });

        let compute_pipeline = device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
            label: None,
            layout: Some(&compute_pipeline_layout),
            module: &shader,
            entry_point: "comp_main",
            compilation_options: PipelineCompilationOptions::default(),
        });

        let texture = device.create_texture(&wgpu::TextureDescriptor {
            format: output_format,
            label: None,
            size: output_size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            usage: wgpu::TextureUsages::COPY_DST | wgpu::TextureUsages::RENDER_ATTACHMENT,
            view_formats: &[],
        });

        let texture_view = texture.create_view(&wgpu::TextureViewDescriptor::default());

        Self {
            pipeline: compute_pipeline,
            texture,
            texture_view,
        }
    }

    pub fn compute(&self) {}
}
