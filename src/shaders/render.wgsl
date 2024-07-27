@vertex
fn vs_main(
    @builtin(vertex_index) in_vertex_index: u32,
) -> @builtin(position) vec4<f32> {
    var positions = array<vec2<f32>, 4>(
        vec2<f32>(-1.0, -1.0),
        vec2<f32>(1.0, -1.0),
        vec2<f32>(-1.0, 1.0),
        vec2<f32>(1.0, 1.0),
    );

    return vec4<f32>(positions[in_vertex_index % 4], 0.0, 1.0);
}

@group(0) @binding(0) var compute_texture: texture_2d<f32>;
@group(0) @binding(1) var compute_sampler: sampler;

@fragment
fn fs_main(@builtin(position) frag_pos: vec4<f32>) -> @location(0) vec4<f32> {
    let tex_coords: vec2<f32> = frag_pos.xy / vec2<f32>(textureDimensions(compute_texture).xy);
    return textureSample(compute_texture, compute_sampler, tex_coords);
}
