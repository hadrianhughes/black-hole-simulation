use crate::hit::{Hit, Hittable};
use crate::vec3;
use crate::ray::Ray;

pub struct Sphere {
    center: vec3::Point3,
    radius: f64,
}

impl Sphere {
    pub fn new(center: vec3::Point3, radius: f64) -> Self {
        Sphere {
            center,
            radius,
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

        let in_range = |n: f64| -> bool { n > t_min && n < t_max };

        // Choose the nearest root
        let sqrt_d = f64::sqrt(discriminant);
        let neg_root = (-half_b - sqrt_d) / a;
        if in_range(neg_root) {
            return Some(neg_root);
        }

        let pos_root = (-half_b + sqrt_d) / a;
        if in_range(pos_root) {
            return Some(pos_root);
        }

        None
    }
}
