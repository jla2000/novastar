@group(0) @binding(0) var output_texture: texture_storage_2d<rgba8unorm, write>;

struct Ray {
    origin: vec3<f32>,
    direction: vec3<f32>,
}

struct Box {
  min: vec3<f32>,
  max: vec3<f32>,
}

fn walk_ray(ray: Ray, distance: f32) -> vec3<f32> {
    return ray.origin + distance * ray.direction;
}

fn hit_sphere(center: vec3<f32>, radius: f32, ray: Ray) -> f32 {
    let oc = center - ray.origin;
    let a = dot(ray.direction, ray.direction);
    let b = -2.0 * dot(ray.direction, oc);
    let c = dot(oc, oc) - radius * radius;
    let discriminant = b * b - 4 * a * c;

    if discriminant < 0 {
        return -1.0;
    } else {
        return (-b - sqrt(discriminant)) / (2.0 * a);
    }
}

fn intersect_box(box: Box, ray: Ray) -> f32 {
    let inv_direction = 1.0 / ray.direction;
    let t1 = (box.min - ray.origin) * inv_direction;
    let t2 = (box.max - ray.origin) * inv_direction;

    let t_min = max(max(min(t1.x, t2.x), min(t1.y, t2.y)), min(t1.z, t2.z));
    let t_max = min(min(max(t1.x, t2.x), max(t1.y, t2.y)), max(t1.z, t2.z));

    return select(-1.0, t_min, t_max >= max(t_min, 0.0));
}

fn trace_ray(ray: Ray) -> vec3<f32> {
    let pos = vec3<f32>(0, 0, 2);

    let dist = intersect_box(Box(vec3<f32>(-0.5, -0.5, -0.5) + pos, vec3<f32>(0.5, 0.5, 0.5) + pos), ray);
    if dist > 0 {
        return vec3<f32>(dist);
    }

    let unit_direction = normalize(ray.direction);
    let a = 0.5 * (unit_direction.y + 1.0);
    return (1.0 - a) * vec3<f32>(1.0, 1.0, 1.0) + a * vec3<f32>(0.5, 0.7, 1.0);
}

@compute
@workgroup_size(10, 10) 
fn comp_main(@builtin(global_invocation_id) id: vec3<u32>) {
    let viewport_size = vec2<f32>(textureDimensions(output_texture).xy);

    let uv = vec2<f32>(id.xy) / viewport_size;
    let aspect = viewport_size.x / viewport_size.y;
    let ndc = vec2<f32>((2 * uv.x - 1) * aspect, 1 - 2 * uv.y);
    let view = vec3<f32>(ndc, 1.0);

    let ray = Ray(vec3<f32>(0.0, 0.0, 0.0), view);
    let output_color = trace_ray(ray);

    textureStore(output_texture, vec2<i32>(id.xy), vec4<f32>(output_color, 1.0));
}
