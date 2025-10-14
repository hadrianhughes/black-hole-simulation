use crate::camera::Camera;
use crate::raytracer::RayTracer;
use crate::sphere::Sphere;

use wgpu;
use winit::window::Window;

pub struct SimpleRayTracer<'window> {
    image_width: u32,
    image_height: u32,
    objects: Vec<Box<Sphere>>,
    camera: Camera,

    surface: wgpu::Surface<'window>,
    device: wgpu::Device,
    queue: wgpu::Queue,
    pipeline: wgpu::ComputePipeline,
}

impl<'window> SimpleRayTracer<'window> {
    pub async fn new(
        image_width: u32,
        image_height: u32,
        window: &'window Window,
        objects: Vec<Box<Sphere>>,
        camera: Camera,
    ) -> Self {
        let instance = wgpu::Instance::default();
        let surface = instance.create_surface(&window).unwrap();
        let adapter = instance.request_adapter(&wgpu::RequestAdapterOptions::default()).await.unwrap();
        let (device, queue) = adapter.request_device(&wgpu::DeviceDescriptor::default()).await.unwrap();

        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("RayTracer shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("shader.wgsl").into()), // TODO: use correct shader path
        });

        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::COMPUTE,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Storage { read_only: false },
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            }],
            label: None,
        });

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("RayTracer pipeline layout"),
            bind_group_layouts: &[&bind_group_layout],
            push_constant_ranges: &[],
        });

        let pipeline = device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
            label: Some("RayTracer pipeline"),
            layout: Some(&pipeline_layout),
            module: &shader,
            entry_point: Some("main"),
            compilation_options: wgpu::PipelineCompilationOptions {
                constants: &[],
                zero_initialize_workgroup_memory: false,
            },
            cache: None,
        });

        let camera_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Camera buffer"),
            size: std::mem::size_of::<Camera>() as u64,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        SimpleRayTracer {
            image_width,
            image_height,
            objects,
            camera,
            surface,
            device,
            queue,
            pipeline,
        }
    }
}

impl<'window> RayTracer for SimpleRayTracer<'window> {
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
            // TODO: set up the bind group
            //pass.set_bind_group(0, &self.bind_group, &[]);
            pass.dispatch_workgroups(
                (self.image_width + 7) / 8,
                (self.image_height + 7) / 8,
                1,
            );
        }

        self.queue.submit(Some(encoder.finish()));
    }
}
