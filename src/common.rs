use rand::Rng;
use std::f32::consts::PI;

pub fn in_range(min: f32, x: f32, max: f32) -> bool {
    x > min && x < max
}

pub fn clamp(min: f32, x: f32, max: f32) -> f32 {
    if x < min {
        return min;
    }
    if x > max {
        return max;
    }
    x
}

pub fn random_in_range(min: f32, max: f32) -> f32 {
    min + (max - min) * rand::rng().random::<f32>()
}

pub fn degrees_to_radians(deg: f32) -> f32 {
    deg * PI / 180.0
}
