use bytemuck::{Pod, Zeroable};

use crate::vec3::Vec3;

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
pub struct Material {
    mat_type: u32,
    color: Vec3,
    emission_intensity: f32,
    refractive_index: f32,
    fuzz: f32,
}

pub fn lambertian(color: Vec3) -> Material {
    Material {
        mat_type: 0,
        color,
        emission_intensity: -1.0,
        refractive_index: -1.0,
        fuzz: -1.0,
    }
}

pub fn metal(color: Vec3, fuzz: f32) -> Material {
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
        color: Vec3::new(0.0, 0.0, 0.0),
        emission_intensity: -1.0,
        refractive_index,
        fuzz: -1.0,
    }
}

pub fn diffuse_light(color: Vec3, emission_intensity: f32) -> Material {
    Material {
        mat_type: 3,
        color,
        emission_intensity,
        refractive_index: -1.0,
        fuzz: -1.0,
    }
}
