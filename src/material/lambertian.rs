use crate::color::Color;
use crate::hit::Hit;
use crate::material::{Material, ScatterResult};
use crate::ray::Ray;
use crate::vec3;

#[derive(Clone)]
pub struct Lambertian {
    albedo: Color,
}

impl Lambertian {
    pub fn new(albedo: Color) -> Self {
        Lambertian { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(&self, _: &Ray, hit: &Hit) -> Option<ScatterResult> {
        let mut scatter_direction = hit.normal + vec3::random_unit_vector();

        if scatter_direction.is_near_zero() {
            scatter_direction = hit.normal;
        }

        Some(ScatterResult {
            ray: Ray::new(hit.position, scatter_direction),
            attenuation: self.albedo,
        })
    }

    fn emit(&self, _r: &Ray, _h: &Hit) -> Color {
        Color::default()
    }
}
