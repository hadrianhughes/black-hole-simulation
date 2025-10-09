use crate::color::Color;
use crate::hit::Hit;
use crate::material::{Material, ScatterResult};
use crate::ray::Ray;

#[derive(Clone)]
pub struct DiffuseLight {
    color: Color,
    intensity: f64,
}

impl DiffuseLight {
    pub fn new(color: Color, intensity: f64) -> Self {
        DiffuseLight {
            color,
            intensity,
        }
    }
}

impl Material for DiffuseLight {
    fn scatter(&self, _ray: &Ray, _hit: &Hit) -> Option<ScatterResult> {
        None
    }

    fn emit(&self, _r: &Ray, _h: &Hit) -> Color {
        self.color * self.intensity
    }
}
