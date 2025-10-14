use crate::camera::Camera;
use crate::raytracer::RayTracer;
use crate::sphere::Sphere;
use wgpu;

pub struct SimpleRayTracer {
    image_width: u32,
    image_height: u32,
    objects: Vec<Box<Sphere>>,
    camera: Camera,

    device: wgpu::Device,
    queue: wgpu::Queue,
    pipeline: wgpu::ComputePipeline,
    bind_group: wgpu::BindGroup,
}

impl SimpleRayTracer {
    pub fn new(camera: Camera, objects: Vec<Box<Sphere>>) -> Self {
        SimpleRayTracer {
            objects,
            camera,
        }
    }
}

impl RayTracer for SimpleRayTracer {
    fn get_camera(self) -> Camera {
        self.camera
    }

    fn render(&self) {
        let mut encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("RayTracer encoder"),
        });

        {
            let mut pass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor {
                label: Some("RayTracer pass"),
                timestamp_writes: None,
            });

            pass.set_pipeline(&self.pipeline);
            pass.set_bind_group(0, &self.bind_group, &[]);
            pass.dispatch_workgroups(
                (self.image_width + 7) / 8,
                (self.image_height + 7) / 8,
                1,
            );
        }

        self.queue.submit(Some(encoder.finish()));
    }
}
