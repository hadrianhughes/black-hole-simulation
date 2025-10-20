use bytemuck::{Pod, Zeroable};

use crate::material::Material;
use crate::vec3;

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
pub struct Sphere {
    center: vec3::Point3,
    radius: f32,
    material: Material,
}

impl Sphere {
    pub fn new(center: vec3::Point3, radius: f32, material: Material) -> Self {
        Sphere {
            center,
            radius,
            material,
        }
    }
}
