struct VertexOutput {
  @builtin(position) clip_position: vec4<f32>,
};

@vertex
fn vs_main(
    @builtin(vertex_index) in_vertex_index: u32,
) -> VertexOutput {
    var out: VertexOutput;
    let x = f32(1 - i32(in_vertex_index)) * 0.5;
    let y = f32(i32(in_vertex_index & 1u) * 2 - 1) * 0.5;
    out.clip_position = vec4<f32>(x, y, 0.0, 1.0);
    return out;
}

@group(0) @binding(0) var compute_texture: texture_2d<f32>;
@group(0) @binding(1) var compute_sampler: sampler;

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
   return textureSample(compute_texture, compute_sampler, in.clip_position.xy / vec2<f32>(800, 600));
}
