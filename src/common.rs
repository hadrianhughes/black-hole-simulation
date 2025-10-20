use rand::Rng;
use std::f32::consts::PI;

pub fn random_in_range(min: f32, max: f32) -> f32 {
    min + (max - min) * rand::rng().random::<f32>()
}

pub fn degrees_to_radians(deg: f32) -> f32 {
    deg * PI / 180.0
}
