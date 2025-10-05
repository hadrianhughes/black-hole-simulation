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
    pub fn new() -> Self {
        Default::default()
    }

    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: Vec3) {
        self.front_face = vec3::dot(r.direction(), outward_normal) < 0.0;
        self.normal = if self.front_face { outward_normal } else { -outward_normal };
    }
}

pub trait Hittable {
    fn hit_scan(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<Hit>;
}

#[derive(Default)]
pub struct HittableList {
    objects: Vec<Box<dyn Hittable>>,
}

impl HittableList {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn add(&mut self, object: Box<dyn Hittable>) {
        self.objects.push(object);
    }
}

impl Hittable for HittableList {
    fn hit_scan(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<Hit> {
        let mut hit = Hit::new();
        let mut did_hit = false;
        let mut closest_t = t_max;

        for object in &self.objects {
            if let Some(h) = object.hit_scan(ray, t_min, closest_t) {
                did_hit = true;
                closest_t = h.t;
                hit = h;
            }
        }

        if did_hit { Some(hit) } else { None }
    }
}
