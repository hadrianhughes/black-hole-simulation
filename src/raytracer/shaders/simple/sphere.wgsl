struct Sphere {
  center: vec3<f32>,
  radius: f32,
  material: Material,
}

fn hit_sphere(object: Sphere, ray: Ray, t_min: f32, t_max: f32) -> Hit {
  let origin_to_center = ray.origin - object.center;

  let a = dot(ray.direction, ray.direction);
  let half_b = dot(ray.direction, origin_to_center);
  let c = dot(origin_to_center, origin_to_center) - object.radius * object.radius;

  var hit: Hit;

  let discriminant = half_b * half_b - a * c;

  if (discriminant < 0.0) {
    hit.hit = false;
    return hit;
  }

  var root: f32;
  var has_root: bool;

  let sqrt_d = sqrt(discriminant);
  let neg_root = (-half_b - sqrt_d) / a;
  if (neg_root > t_min && neg_root < t_max) {
    root = neg_root;
    has_root = true;
  } else {
    let pos_root = (-half_b + sqrt_d) / a;
    if (pos_root > t_min && pos_root < t_max) {
      root = pos_root;
      has_root = true;
    } else {
      has_root = false;
    }
  }

  if (!has_root) {
    hit.hit = false;
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
  var hit: Hit;
  var closest_t = t_max;

  for (var i: u32 = 0u;i < arrayLength(&objects);i = i + 1u) {
    let sphere = objects[i];
    let h = hit_sphere(sphere, ray, t_min, closest_t);
    if (h.hit) {
      closest_t = h.t;
      hit = h;
    }
  }

  return hit;
}
