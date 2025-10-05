use crate::ray::Ray;
use crate::vec3;
use crate::vec3::{Point3, Vec3};

#[derive(Clone, Default)]
pub struct Hit {
    pub t: f64,
    pub position: Point3,
    pub front_face: bool,
    pub normal: Vec3,
}

impl Hit {
    pub fn new(t: f64, position: Point3) -> Self {
        let mut hit: Hit = Default::default();
        hit.t = t;
        hit.position = position;
        hit
    }

    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: Vec3) {
        self.front_face = vec3::dot(r.direction(), outward_normal) < 0.0;
        self.normal = if self.front_face { outward_normal } else { -outward_normal };
    }
}

pub trait Hittable {
    fn hit_scan(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<Hit>;
}
