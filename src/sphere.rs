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

        // Choose the nearest root
        let root: Option<f64> = {
            let in_range = |n: f64| -> bool { n > t_min && n < t_max };

            let sqrt_d = f64::sqrt(discriminant);
            let neg_root = (-half_b - sqrt_d) / a;
            if in_range(neg_root) {
                Some(neg_root)
            } else {
                let pos_root = (-half_b + sqrt_d) / a;
                if in_range(pos_root) {
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

                let outward_normal = (hit.position - self.center) / self.radius;
                hit.set_face_normal(ray, outward_normal);

                Some(hit)
            },
            None => { None }
        }
    }
}
