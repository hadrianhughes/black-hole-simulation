use crate::ray::Ray;

pub type Hit = f64;

pub trait Hittable {
    fn hit_scan(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<Hit>;
}
