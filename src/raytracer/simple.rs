use bytemuck;
use wgpu;
use winit::window::Window;

use crate::camera::Camera;
use crate::raytracer::RayTracer;
use crate::sphere::Sphere;

pub struct SimpleRayTracer<'window> {
    image_width: u32,
    image_height: u32,
    objects: Vec<Sphere>,
    camera: Camera,

    surface: wgpu::Surface<'window>,
    device: wgpu::Device,
    queue: wgpu::Queue,
    pipeline: wgpu::ComputePipeline,
    bind_group_layout: wgpu::BindGroupLayout,
}

impl<'window> SimpleRayTracer<'window> {
    pub async fn new(
        image_width: u32,
        image_height: u32,
        window: &'window Window,
        objects: Vec<Sphere>,
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

        SimpleRayTracer {
            image_width,
            image_height,
            objects,
            camera,
            surface,
            device,
            queue,
            pipeline,
            bind_group_layout,
        }
    }
}

impl<'window> RayTracer for SimpleRayTracer<'window> {
    fn render(&self) -> Result<(), wgpu::SurfaceError> {
        let output = self.surface.get_current_texture()?;
        let view = output.texture.create_view(&wgpu::TextureViewDescriptor::default());

        let camera_buffer = self.device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Camera buffer"),
            size: std::mem::size_of::<Camera>() as u64,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        self.queue.write_buffer(&camera_buffer, 0, bytemuck::cast_slice(&[self.camera]));

        let objects_buffer = self.device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Objects buffer"),
            size: (std::mem::size_of::<Sphere>() * self.objects.len()) as u64,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        self.queue.write_buffer(&objects_buffer, 1, bytemuck::cast_slice(self.objects.as_slice()));

        let bind_group = self.device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &self.bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: camera_buffer.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: objects_buffer.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 2,
                    resource: wgpu::BindingResource::TextureView(&view),
                },
            ],
            label: Some("RayTracer BindGroup"),
        });

        let mut encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("RayTracer encoder"),
        });

        {
            let mut pass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor {
                label: Some("RayTracer pass"),
                timestamp_writes: None,
            });

            pass.set_pipeline(&self.pipeline);
            pass.set_bind_group(0, &bind_group, &[]);
            pass.dispatch_workgroups(
                (self.image_width + 7) / 8,
                (self.image_height + 7) / 8,
                1,
            );
        }

        self.queue.submit(Some(encoder.finish()));

        Ok(())
    }
}
