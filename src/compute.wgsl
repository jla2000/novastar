@group(0) @binding(0) var output_texture: texture_storage_2d<rgba8unorm, write>;

@compute
@workgroup_size(16, 16) 
fn comp_main(@builtin(global_invocation_id) id: vec3<u32>) {
    textureStore(output_texture, vec2<i32>(id.xy), vec4<f32>(1.0, 0.0, 0.0, 1.0));
}
