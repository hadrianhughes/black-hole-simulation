use bytemuck::{Pod, Zeroable};

use crate::vec3::Vec3;

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
pub struct Material {
    mat_type: u32,
    _pad0: [u32; 3],
    color: Vec3,
    emission_intensity: f32,
    refractive_index: f32,
    fuzz: f32,
    _pad1: f32,
}

impl Material {
    pub fn new(
        mat_type: u32,
        color: Vec3,
        emission_intensity: f32,
        refractive_index: f32,
        fuzz: f32,
    ) -> Self {
        Material {
            mat_type,
            _pad0: [0, 0, 0],
            color,
            emission_intensity,
            refractive_index,
            fuzz,
            _pad1: 0.0,
        }
    }
}

pub fn lambertian(color: Vec3) -> Material {
    Material::new(0, color, -1.0, -1.0, -1.0)
}

pub fn metal(color: Vec3, fuzz: f32) -> Material {
    Material::new(1, color, -1.0, -1.0, fuzz)
}

pub fn dielectric(refractive_index: f32) -> Material {
    Material::new(2, Vec3::new(0.0, 0.0, 0.0), -1.0, refractive_index, -1.0)
}

pub fn diffuse_light(color: Vec3, emission_intensity: f32) -> Material {
    Material::new(3, color, emission_intensity, -1.0, -1.0)
}
