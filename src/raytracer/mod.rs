pub mod simple;

use wgpu;

pub trait RayTracer {
    fn render(&self) -> Result<(), wgpu::SurfaceError>;
}
