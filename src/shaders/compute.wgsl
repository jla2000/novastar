@group(0) @binding(0) var output_texture: texture_storage_2d<rgba8unorm, write>;

@compute
@workgroup_size(10, 10) 
fn comp_main(@builtin(global_invocation_id) id: vec3<u32>) {
    let red = vec3<f32>(1.0, 0.0, 0.0);
    let green = vec3<f32>(0.0, 1.0, 0.0);
    let blue = vec3<f32>(0.0, 0.0, 1.0);

    let texture_size = vec2<f32>(textureDimensions(output_texture).xy);
    let uv = vec2<f32>(id.xy) / texture_size;

    let gradient: vec3<f32> = mix(red, green, uv.x);
    let output_color = mix(gradient, blue, uv.y);

    textureStore(output_texture, vec2<i32>(id.xy), vec4<f32>(output_color, 1.0));
}
