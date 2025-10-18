struct Config {
  image_width: u32,
  image_height: u32,
}

struct Hit {
  hit: bool,
  t: f32,
  position: vec3<f32>,
  front_face: bool,
  material: Material,
}

@group(0) @binding(0)
var <storage, read> camera: Camera;

@group(0) @binding(1)
var <storage, read> objects: array<Sphere>;

@group(0) @binding(2)
var <storage, read> config: array<f32>;

@group(0) @binding(3)
var <storage, read> output: array<f32>;

fn ray_color(ray: Ray, depth: i32) -> vec3<f32> {
  if (depth <= 0) {
    return vec3<f32>(0.0, 0.0, 0.0);
  }

  let hit = hit_scan(objects, ray, 0.001, 1.0 / 0.0);
  if (hit.hit) {
  }
}

@compute @workgroup_size(8, 8)
fn main(@builtin(global_invocation_id) global_id: vec3<u32>) {
  let index = global_id.y * config.image_width + global_id.x;
}
