pub mod simple;

use crate::camera::Camera;

pub trait RayTracer {
    fn get_camera(self) -> Camera;
    fn render(&self);
}
