use crate::Ray;
use crate::vec3::{Point3, Vec3};

pub struct Camera {
    origin: Point3,
    bottom_left: Point3,
    horizontal: Point3,
    vertical: Point3,
}

impl Camera {
    pub fn new(aspect_ratio: f64) -> Self {
        let viewport_height = 2.0;
        let viewport_width = viewport_height * aspect_ratio;
        let focal_length = 1.0;

        let origin = Point3::new(0.0, 0.0, 0.0);
        let horizontal = Point3::new(viewport_width, 0.0, 0.0);
        let vertical = Point3::new(0.0, viewport_height, 0.0);

        let bottom_left = origin - horizontal / 2.0 - vertical / 2.0 - Vec3::new(0.0, 0.0, focal_length);

        Camera {
            origin,
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
