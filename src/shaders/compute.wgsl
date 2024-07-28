@group(0) @binding(0) var output_texture: texture_storage_2d<rgba8unorm, write>;

struct Ray {
    origin: vec3<f32>,
    direction: vec3<f32>,
}

fn walk_ray(ray: Ray, distance: f32) -> vec3<f32> {
    return ray.origin + distance * ray.direction;
}

fn trace_ray(ray: Ray) -> vec3<f32> {
    let unit_direction = normalize(ray.direction);
    let a = 0.5 * (unit_direction.y + 1.0);
    return (1.0 - a) * vec3<f32>(1.0, 1.0, 1.0) + a * vec3<f32>(0.5, 0.7, 1.0);
}

@compute
@workgroup_size(10, 10) 
fn comp_main(@builtin(global_invocation_id) id: vec3<u32>) {
    let viewport_size = vec2<f32>(textureDimensions(output_texture).xy);

    let uv = vec2<f32>(id.xy) / viewport_size;
    let ndc = vec2<f32>(2 * uv.x - 1, 1 - 2 * uv.y);
    let view = vec3<f32>(ndc, 1.0);

    let ray = Ray(vec3<f32>(0.0, 0.0, 0.0), view);
    let output_color = trace_ray(ray);

    textureStore(output_texture, vec2<i32>(id.xy), vec4<f32>(output_color, 1.0));
}
