use crate::color::Color;
use crate::hit::Hit;
use crate::material::{Material, ScatterResult};
use crate::ray::Ray;
use crate::vec3;

pub struct Metal {
    albedo: Color,
}

impl Metal {
    pub fn new(albedo: Color) -> Self {
        Metal { albedo }
    }
}

impl Material for Metal {
    fn scatter(&self, ray: &Ray, hit: &Hit) -> Option<ScatterResult> {
        let reflected = vec3::reflect(ray.direction().unit(), hit.normal);

        let scattered_ray = Ray::new(hit.position, reflected);

        let did_scatter = vec3::dot(scattered_ray.direction(), hit.normal) > 0.0;
        if did_scatter {
            Some(ScatterResult {
                ray: scattered_ray,
                attenuation: self.albedo,
            })
        } else {
            None
        }
    }
}
