use bytemuck::{Pod, Zeroable};

use crate::color::Color;

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
pub struct Material {
    mat_type: u32,
    color: Color,
    emission_intensity: f32,
    refractive_index: f32,
    fuzz: f32,
}

pub fn lambertian(color: Color) -> Material {
    Material {
        mat_type: 0,
        color,
        emission_intensity: -1.0,
        refractive_index: -1.0,
        fuzz: -1.0,
    }
}

pub fn metal(color: Color, fuzz: f32) -> Material {
    Material {
        mat_type: 1,
        color,
        emission_intensity: -1.0,
        refractive_index: -1.0,
        fuzz,
    }
}

pub fn dielectric(refractive_index: f32) -> Material {
    Material {
        mat_type: 2,
        color: Color::new(0.0, 0.0, 0.0),
        emission_intensity: -1.0,
        refractive_index,
        fuzz: -1.0,
    }
}

pub fn diffuse_light(color: Color, emission_intensity: f32) -> Material {
    Material {
        mat_type: 3,
        color,
        emission_intensity,
        refractive_index: -1.0,
        fuzz: -1.0,
    }
}
