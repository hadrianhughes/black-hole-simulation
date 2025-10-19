struct Material {
  mat_type: u32,
  color: vec3<f32>,
  emission_intensity: f32,
  refractive_index: f32,
  fuzz: f32,
}

struct ScatterResult {
  scattered: bool,
  ray: Ray,
  attenuation: vec3<f32>,
}

fn scatter(material: Material, ray: Ray, hit: Hit) -> ScatterResult {
  switch (material.mat_type) {
    // Lambertian
    case 0: {
      var scatter_direction = hit.normal + random_unit_vec3(config.rand_seed);

      let EPS: f32 = 1.0e-18;
      let near_zero = abs(scatter_direction.x) < EPS && abs(scatter_direction.y) < EPS && abs(scatter_direction.z) < EPS;

      if (near_zero) {
        scatter_direction = hit.normal;
      }

      return ScatterResult(true, Ray(hit.position, scatter_direction), material.color);
    }
    // Metal
    case 1: {
      let reflected = reflect(normalize(ray.direction), hit.normal);
      let scattered_ray = Ray(hit.position, reflected + material.fuzz * random_in_unit_sphere(config.rand_seed));

      let did_scatter = dot(scattered_ray.direction, hit.normal) > 0.0;
      if (did_scatter) {
        return ScatterResult(true, scattered_ray, material.color);
      } else {
        return ScatterResult(false);
      }
    }
    // Dielectric
    case 2: {
      var refractive_ratio: f32;
      if (hit.front_face) {
        refractive_ratio = 1.0 / material.refractive_index;
      } else {
        refractive_ratio = material.refractive_index;
      }

      let unit_direction = normalize(ray.direction);
      let cos_theta = min(dot(-unit_direction, hit.normal), 1.0);
      let sin_theta = sqrt(1.0 - cos_theta * cos_theta);

      let cannot_refract = refractive_ratio * sin_theta > 1.0;

      var reflectance: f32;
      var r0 = (1.0 - material.refractive_ratio) / (1.0 + material.refractive_ratio);
      r0 = r0 * r0;
      reflectance = r0 + (1.0 - r0) * pow(1.0 - cos_theta, 5.0);

      let is_shallow_angle = reflectance > rand(config.rand_seed);

      var direction: vec3<f32>;
      if (cannot_refract || is_shallow_angle) {
        direction = reflect(unit_direction, hit.normal);
      } else {
        direction = refract(unit_direction, hit.normal, refractive_ratio);
      }

      return ScatterResult(true, Ray(hit.position, direction), vec3<f32>(1.0, 1.0, 1.0));
    }
    // Diffuse light
    default: {
      return ScatterResult(false);
    }
  }
}

fn emit(material: Material) -> vec3<f32> {
  switch (material.mat_type) {
    case 3: {
      return material.color * material.emission_intensity;
    }
    default: {
      return vec3<f32>(0.0, 0.0, 0.0);
    }
  }
}
