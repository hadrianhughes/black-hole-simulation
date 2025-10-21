struct Hit {
  hit: bool,
  t: f32,
  position: vec3<f32>,
  normal: vec3<f32>,
  front_face: bool,
  material: Material,
}

struct Sphere {
  center: vec3<f32>,
  radius: f32,
  material: Material,
}

fn default_hit() -> Hit {
  return Hit(
    false,
    0.0,
    vec3<f32>(0.0),
    vec3<f32>(0.0),
    false,
    Material(0, vec3<f32>(0.0), 0.0, 0.0, 0.0)
  );
}

fn hit_sphere(object: Sphere, ray: Ray, t_min: f32, t_max: f32) -> Hit {
  let origin_to_center = ray.origin - object.center;

  let a = dot(ray.direction, ray.direction);
  let half_b = dot(ray.direction, origin_to_center);
  let c = dot(origin_to_center, origin_to_center) - object.radius * object.radius;

  var hit = default_hit();

  let discriminant = half_b * half_b - a * c;

  if (discriminant < 0.0) {
    return hit;
  }

  var root: f32;

  let sqrt_d = sqrt(discriminant);
  let neg_root = (-half_b - sqrt_d) / a;
  if (neg_root > t_min && neg_root < t_max) {
    root = neg_root;
    hit.hit = true;
  } else {
    let pos_root = (-half_b + sqrt_d) / a;
    if (pos_root > t_min && pos_root < t_max) {
      root = pos_root;
      hit.hit = true;
    }
  }

  if (!hit.hit) {
    return hit;
  }

  hit.t = root;
  hit.position = ray_at(ray, hit.t);
  hit.material = object.material;

  let outward_normal = (hit.position - object.center) / object.radius;
  hit.front_face = dot(ray.direction, outward_normal) < 0.0;

  if (hit.front_face) {
    hit.normal = outward_normal;
  } else {
    hit.normal = -outward_normal;
  }

  return hit;
}

fn hit_scan(ray: Ray, t_min: f32, t_max: f32) -> Hit {
  var hit = default_hit();
  var closest_t = t_max;

  for (var i: u32 = 0u;i < config.object_count;i = i + 1u) {
    let sphere = objects[i];
    let h = hit_sphere(sphere, ray, t_min, closest_t);
    if (h.hit) {
      closest_t = h.t;
      hit = h;
    }
  }

  return hit;
}
