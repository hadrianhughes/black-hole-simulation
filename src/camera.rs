use crate::common;
use crate::Ray;
use crate::vec3;
use crate::vec3::{Point3, Vec3};

pub struct Camera {
    origin: Point3,
    bottom_left: Point3,
    horizontal: Point3,
    vertical: Point3,
}

impl Camera {
    pub fn new(
        look_from: Point3,
        look_at: Point3,
        up: Vec3,
        fov_deg: f64,
        aspect_ratio: f64,
    ) -> Self {
        let theta = common::degrees_to_radians(fov_deg);

        let viewport_height = 2.0 * f64::tan(theta / 2.0);
        let viewport_width = viewport_height * aspect_ratio;

        let w = (look_from - look_at).unit();
        let u = vec3::cross(up, w).unit();
        let v = vec3::cross(w, u);

        let horizontal = viewport_width * u;
        let vertical = viewport_height * v;

        let bottom_left = look_from - horizontal / 2.0 - vertical / 2.0 - w;

        Camera {
            origin: look_from,
            bottom_left,
            horizontal,
            vertical
        }
    }

    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        Ray::new(
            self.origin,
            self.bottom_left + u * self.horizontal + v * self.vertical - self.origin
        )
    }
}
