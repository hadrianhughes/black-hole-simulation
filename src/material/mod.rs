pub mod dielectric;
pub mod lambertian;
pub mod metal;

use crate::color::Color;
use crate::hit::Hit;
use crate::ray::Ray;

pub use dielectric::Dielectric;
pub use lambertian::Lambertian;
pub use metal::Metal;

pub struct ScatterResult {
    pub ray: Ray,
    pub attenuation: Color,
}

pub trait Material {
    fn scatter(&self, ray: &Ray, hit: &Hit) -> Option<ScatterResult>;
}
