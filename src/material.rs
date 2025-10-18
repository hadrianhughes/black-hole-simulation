use bytemuck::{Pod, Zeroable};

use crate::vec3::Color;

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
struct Material {
    mat_type: u32,
    color: Color,
    emission_intensity: f32,
    refractive_index: f32,
    fuzz: f32,
}
