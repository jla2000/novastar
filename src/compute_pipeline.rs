use wgpu::PipelineCompilationOptions;

pub struct ComputePipeline {
    pipeline: wgpu::ComputePipeline,
    texture: wgpu::Texture,
    texture_view: wgpu::TextureView,
    bind_group: wgpu::BindGroup,
}

impl ComputePipeline {
    pub fn new(device: &wgpu::Device, output_size: wgpu::Extent3d) -> Self {
        let shader = device.create_shader_module(wgpu::include_wgsl!("compute.wgsl"));

        let texture = device.create_texture(&wgpu::TextureDescriptor {
            format: wgpu::TextureFormat::Rgba8Unorm,
            label: None,
            size: output_size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT
                | wgpu::TextureUsages::STORAGE_BINDING
                | wgpu::TextureUsages::TEXTURE_BINDING,
            view_formats: &[],
        });

        let texture_view = texture.create_view(&wgpu::TextureViewDescriptor::default());

        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: None,
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::COMPUTE,
                ty: wgpu::BindingType::StorageTexture {
                    access: wgpu::StorageTextureAccess::WriteOnly,
                    format: wgpu::TextureFormat::Rgba8Unorm,
                    view_dimension: wgpu::TextureViewDimension::D2,
                },
                count: None,
            }],
        });

        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: None,
            layout: &bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: wgpu::BindingResource::TextureView(&texture_view),
            }],
        });

        let compute_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: None,
                bind_group_layouts: &[&bind_group_layout],
                push_constant_ranges: &[],
            });

        let compute_pipeline = device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
            label: None,
            layout: Some(&compute_pipeline_layout),
            module: &shader,
            entry_point: "comp_main",
            compilation_options: PipelineCompilationOptions::default(),
        });

        Self {
            pipeline: compute_pipeline,
            texture,
            texture_view,
            bind_group,
        }
    }

    pub fn compute<'a>(&'a self, compute_pass: &mut wgpu::ComputePass<'a>) {
        compute_pass.set_pipeline(&self.pipeline);
        compute_pass.set_bind_group(0, &self.bind_group, &[]);
        compute_pass.dispatch_workgroups(self.texture.width() / 16, self.texture.height() / 16, 1);
    }

    pub fn get_texture_view(&self) -> &wgpu::TextureView {
        &self.texture_view
    }
}
