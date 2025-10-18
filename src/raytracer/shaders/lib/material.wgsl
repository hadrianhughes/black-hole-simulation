struct Material {
  type: u32,
  albedo: vec3<f32>,
}

struct ScatterResult {
  scattered: bool,
  ray: Ray,
  attenuation: vec3<f32>,
}

fn scatter(material: Material, ray: Ray, hit: Hit) -> ScatterResult {
  switch (material.type) {
    // Lambertian
    case 0: {
      var scatter_direction = hit.normal + random_unit_vec3(config.rand_seed);

      let EPS: f32 = 1.0e-18;
      let near_zero = abs(scatter_direction.x) < EPS && abs(scatter_direction.y) < EPS && abs(scatter_direction.z) < EPS;

      if (near_zero) {
        scatter_direction = hit.normal;
      }

      return ScatterResult(true, Ray(hit.position, scatter_direction), material.albedo);
    }
  }
}
