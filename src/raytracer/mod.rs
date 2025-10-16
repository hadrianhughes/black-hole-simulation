pub mod simple;

use wgpu;

use crate::camera::Camera;

pub trait RayTracer {
    fn get_camera(self) -> Camera;
    fn render(&self) -> Result<(), wgpu::SurfaceError>;
}
