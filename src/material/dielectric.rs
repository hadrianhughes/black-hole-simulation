use rand::Rng;

use crate::color::Color;
use crate::hit::Hit;
use crate::material::{Material, ScatterResult};
use crate::ray::Ray;
use crate::vec3;

#[derive(Clone)]
pub struct Dielectric {
    ref_idx: f64,
}

impl Dielectric {
    pub fn new(ref_idx: f64) -> Self {
        Dielectric {
            ref_idx
        }
    }

    pub fn reflectance(cos: f64, ref_idx: f64) -> f64 {
        // Schlick approximation
        let mut r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
        r0 = r0 * r0;
        r0 + (1.0 - r0) * f64::powf(1.0 - cos, 5.0)
    }
}

impl Material for Dielectric {
    fn scatter(&self, ray: &Ray, hit: &Hit) -> Option<ScatterResult> {
        let refractive_ratio = if hit.front_face {
            1.0 / self.ref_idx
        } else {
            self.ref_idx
        };

        let unit_direction = ray.direction().unit();
        let cos_theta = f64::min(vec3::dot(-unit_direction, hit.normal), 1.0);
        let sin_theta = f64::sqrt(1.0 - cos_theta * cos_theta);

        let cannot_refract = refractive_ratio * sin_theta > 1.0;
        let is_shallow_angle = Self::reflectance(cos_theta, refractive_ratio) > rand::rng().random::<f64>();

        let direction = if cannot_refract || is_shallow_angle {
            vec3::reflect(unit_direction, hit.normal)
        } else {
            vec3::refract(unit_direction, hit.normal, refractive_ratio)
        };

        Some(ScatterResult {
            ray: Ray::new(hit.position, direction),
            attenuation: Color::new(1.0, 1.0, 1.0),
        })
    }
}
