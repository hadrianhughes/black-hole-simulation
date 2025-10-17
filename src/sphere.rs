use bytemuck::{Pod, Zeroable};

use crate::common::in_range;
use crate::hit::{Hit, Hittable};
use crate::vec3;
use crate::ray::Ray;

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
pub struct Sphere {
    center: vec3::Point3,
    radius: f32,
    material: u32,
}

impl Sphere {
    pub fn new(center: vec3::Point3, radius: f32, material: u32) -> Self {
        Sphere {
            center,
            radius,
            material,
        }
    }
}

impl Hittable for Sphere {
    fn hit_scan(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<Hit> {
        let origin_to_center = ray.origin() - self.center;

        let a = vec3::dot(ray.direction(), ray.direction());
        let half_b = vec3::dot(ray.direction(), origin_to_center);
        let c = vec3::dot(origin_to_center, origin_to_center) - self.radius * self.radius;

        let discriminant = half_b * half_b - a * c;

        // Are there solutions?
        if discriminant < 0.0 {
            return None;
        }

        // Choose the nearest root
        let root: Option<f64> = {
            let sqrt_d = f64::sqrt(discriminant);
            let neg_root = (-half_b - sqrt_d) / a;
            if in_range(t_min, neg_root, t_max) {
                Some(neg_root)
            } else {
                let pos_root = (-half_b + sqrt_d) / a;
                if in_range(t_min, pos_root, t_max) {
                    Some(pos_root)
                } else {
                    None
                }
            }
        };

        match root {
            Some(t) => {
                let mut hit = Hit::new();
                hit.t = t;
                hit.position = ray.at(t);
                hit.material = Some(self.material.clone());

                let outward_normal = (hit.position - self.center) / self.radius;
                hit.set_face_normal(ray, outward_normal);

                Some(hit)
            },
            None => { None }
        }
    }
}
