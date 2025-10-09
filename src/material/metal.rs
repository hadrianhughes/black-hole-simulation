use crate::color::Color;
use crate::hit::Hit;
use crate::material::{Material, ScatterResult};
use crate::ray::Ray;
use crate::vec3;

#[derive(Clone)]
pub struct Metal {
    albedo: Color,
    fuzz: f64,
}

impl Metal {
    pub fn new(albedo: Color, fuzz: f64) -> Self {
        Metal {
            albedo,
            fuzz: f64::min(fuzz, 1.0),
        }
    }
}

impl Material for Metal {
    fn scatter(&self, ray: &Ray, hit: &Hit) -> Option<ScatterResult> {
        let reflected = vec3::reflect(ray.direction().unit(), hit.normal);

        let scattered_ray = Ray::new(hit.position, reflected + self.fuzz * vec3::random_in_unit_sphere());

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

    fn emit(&self, _r: &Ray, _h: &Hit) -> Color {
        Color::default()
    }
}
