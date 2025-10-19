struct Config {
  image_width: u32,
  image_height: u32,
  max_depth: u32,
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

fn ray_color(ray: Ray, max_depth: i32, mut rng: ptr<function, RNG>) -> vec3<f32> {
  var r: Ray = ray;
  var pixel_color = vec3<f32>(0.0, 0.0, 0.0);
  var depth = max_depth;

  while (depth-- > 0) {
    let hit = hit_scan(objects, ray, 0.001, 1.0 / 0.0);
    if (!hit.hit) {
      let unit_direction = normalize(ray.direction);
      let t = 0.5 * (unit_direction.y + 1.0);
      pixel_color += (1.0 - t) * vec3<f32>(1.0, 1.0, 1.0) + t * vec3<f32>(0.5, 0.7, 1.0);
      break;
    }

    let emission = emit(hit.material);

    r = scatter(hit.material, r, hit, &rng);
    if (!r.scattered) {
      pixel_color += emission;
      break;
    }

    pixel_color += emission + r.attenuation;
  }

  return pixel_color;
}

@compute @workgroup_size(8, 8)
fn main(@builtin(global_invocation_id) global_id: vec3<u32>) {
  // Prevent out-of-bounds access
  if (global_id.x >= config.image_width || global_id.y >= config.image_height) {
      return;
  }

  var rng = RNG(seed: hash_coords(global_id.x, global_id.y, uniforms.frame));

  let ray = get_ray(camera, global_id.x, global_id.y);
  let pixel_color = ray_color(ray, config.max_depth, &rng);

  let index = global_id.y * config.image_width + global_id.x;
  let base_index: u32 = pixel_index * 3u;

  output[base_index + 0u] = pixel_index.x;
  output[base_index + 1u] = pixel_index.y;
  output[base_index + 2u] = pixel_index.z;
}
