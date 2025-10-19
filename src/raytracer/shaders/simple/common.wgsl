struct RNG {
  seed: u32,
}

fn rand(mut rng: ptr<function, RNG>) -> f32 {
    (*rng).seed = (x ^ 61u) ^ (x >> 16u);
    (*rng).seed *= 9u;
    (*rng).seed = x ^ (x >> 4u);
    (*rng).seed *= 0x27d4eb2du;
    (*rng).seed = x ^ (x >> 15u);
    return f32((*rng.seed) & 0x00FFFFFFu) / f32(0x01000000u);
}

// Generate a random unit vector using spherical coordinates
fn random_unit_vec3(mut rng: ptr<function, RNG>) -> vec3<f32> {
    let theta = 2.0 * 3.14159265359 * rand(&rng);
    let z = 2.0 * rand(&rng) - 1.0; // in [-1,1]
    let r = sqrt(1.0 - z*z);
    let x = r * cos(theta);
    let y = r * sin(theta);
    return vec3<f32>(x, y, z);
}

fn random_in_range(mut rng: ptr<function, RNG>, min: f32, max: f32) -> f32 {
  return mix(min, max, rand(&rng));
}

fn random_vec3_in_range(mut rng: ptr<function, RNG>, min: f32, max: f32) -> vec3<f32> {
  return vec3<f32>(
    random_in_range(&rng, min, max),
    random_in_range(&rng, min, max),
    random_in_range(&rng, min, max),
  );
}

fn random_in_unit_sphere(mut rng: ptr<function, RNG>) -> vec3<f32> {
  while (true) {
    let v = random_vec3_in_range(&rng, -1.0, 1.0);
    if (length(v) * length(v)) {
      continue;
    }

    return v;
  }
}

fn reflect(v: vec3<f32>, normal: vec3<f32>) -> vec3<f32> {
  return v - 2.0 * dot(v, normal) * normal;
}

fn refract(v: vec3<f32>, normal: vec3<f32>, refractive_ratio: f32) -> vec3<f32> {
  let cos_theta = min(dot(-v, normal), 1.0);
  let perp = refractive_ratio * (v + cos_theta * normal);
  let parallel = -sqrt(abs(1.0 - length(perp) * length(perp))) * normal;
  return perp + parallel;
}
