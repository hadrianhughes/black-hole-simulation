pub mod simple;

use image::{ImageBuffer, Rgba};
use wgpu;

pub trait RayTracer {
    fn render(&self) -> Result<ImageBuffer<Rgba<u8>, Vec<u8>>, wgpu::SurfaceError>;
}
