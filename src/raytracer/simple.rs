use bytemuck;
use bytemuck::{Pod, Zeroable};
use image::{ImageBuffer, Rgba};
use wgpu;

use crate::camera::Camera;
use crate::raytracer::RayTracer;
use crate::sphere::Sphere;

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
struct SimpleRayTracerConfig {
    image_width: u32,
    image_height: u32,
    max_depth: u32,
    object_count: u32,
}

pub struct SimpleRayTracer<'camera> {
    config: SimpleRayTracerConfig,
    objects: Vec<Sphere>,
    camera: &'camera Camera,

    device: wgpu::Device,
    queue: wgpu::Queue,
    pipeline: wgpu::ComputePipeline,
    bind_group_layout: wgpu::BindGroupLayout,
}

impl<'camera> SimpleRayTracer<'camera> {
    pub async fn new(
        image_width: u32,
        image_height: u32,
        objects: Vec<Sphere>,
        camera: &'camera Camera,
    ) -> Self {
        let instance = wgpu::Instance::default();
        let adapter = instance.request_adapter(&wgpu::RequestAdapterOptions::default()).await.unwrap();
        let (device, queue) = adapter.request_device(&wgpu::DeviceDescriptor::default()).await.unwrap();

        let shader_src = concat!(
            include_str!("shaders/simple/common.wgsl"),
            include_str!("shaders/simple/material.wgsl"),
            include_str!("shaders/simple/ray.wgsl"),
            include_str!("shaders/simple/sphere.wgsl"),
            include_str!("shaders/simple/camera.wgsl"),
            include_str!("shaders/simple/main.wgsl"),
        );

        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("RayTracer shader"),
            source: wgpu::ShaderSource::Wgsl(shader_src.into()),
        });

        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::COMPUTE,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Storage { read_only: true },
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::COMPUTE,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Storage { read_only: true },
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 2,
                    visibility: wgpu::ShaderStages::COMPUTE,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Storage { read_only: true },
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 3,
                    visibility: wgpu::ShaderStages::COMPUTE,
                    ty: wgpu::BindingType::StorageTexture {
                        access: wgpu::StorageTextureAccess::WriteOnly,
                        format: wgpu::TextureFormat::Rgba8Unorm,
                        view_dimension: wgpu::TextureViewDimension::D2,
                    },
                    count: None,
                },
            ],
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
            config: SimpleRayTracerConfig {
                image_width,
                image_height,
                max_depth: 50,
                object_count: objects.len() as u32,
            },
            objects,
            camera,
            device,
            queue,
            pipeline,
            bind_group_layout,
        }
    }
}

impl<'camera> RayTracer for SimpleRayTracer<'camera> {
    fn render(&self) -> Result<ImageBuffer<Rgba<u8>, Vec<u8>>, wgpu::SurfaceError> {
        let camera_buffer = self.device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Camera buffer"),
            size: std::mem::size_of::<Camera>() as u64,
            usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        self.queue.write_buffer(&camera_buffer, 0, bytemuck::cast_slice(&[*self.camera]));

        let objects_buffer = self.device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Objects buffer"),
            size: (std::mem::size_of::<Sphere>() * self.objects.len()) as u64,
            usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        self.queue.write_buffer(&objects_buffer, 0, bytemuck::cast_slice(self.objects.as_slice()));

        let parameters_buffer = self.device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Parameters buffer"),
            size: std::mem::size_of::<SimpleRayTracerConfig>() as u64,
            usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        self.queue.write_buffer(&parameters_buffer, 0, bytemuck::cast_slice(&[self.config]));

        let output_texture = self.device.create_texture(&wgpu::TextureDescriptor {
            label: Some("Output texture"),
            size: wgpu::Extent3d {
                width: self.config.image_width,
                height: self.config.image_height,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8Unorm,
            usage: wgpu::TextureUsages::STORAGE_BINDING | wgpu::TextureUsages::COPY_SRC,
            view_formats: &[],
        });
        let output_view = output_texture.create_view(&wgpu::TextureViewDescriptor::default());

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
                    resource: parameters_buffer.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 3,
                    resource: wgpu::BindingResource::TextureView(&output_view),
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
                (self.config.image_width + 7) / 8,
                (self.config.image_height + 7) / 8,
                1,
            );
        }

        let bytes_per_pixel = 4;
        let bytes_per_row = bytes_per_pixel * self.config.image_width;
        let a = wgpu::COPY_BYTES_PER_ROW_ALIGNMENT;
        let padded_bytes_per_row = ((bytes_per_row + a - 1) / a) * a;

        let output_buffer_size = (padded_bytes_per_row * self.config.image_height) as wgpu::BufferAddress;
        let output_buffer = self.device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Output buffer"),
            size: output_buffer_size,
            usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::MAP_READ,
            mapped_at_creation: false,
        });

        encoder.copy_texture_to_buffer(
            wgpu::TexelCopyTextureInfo {
                texture: &output_texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All,
            },
            wgpu::TexelCopyBufferInfo {
                buffer: &output_buffer,
                layout: wgpu::TexelCopyBufferLayout {
                    offset: 0,
                    bytes_per_row: Some(padded_bytes_per_row),
                    rows_per_image: Some(self.config.image_height),
                },
            },
            wgpu::Extent3d {
                width: self.config.image_width,
                height: self.config.image_height,
                depth_or_array_layers: 1,
            },
        );

        self.queue.submit(Some(encoder.finish()));

        let buffer_slice = output_buffer.slice(..);
        buffer_slice.map_async(wgpu::MapMode::Read, |_| ());
        self.device.poll(wgpu::PollType::Wait {
            submission_index: None,
            timeout: Some(std::time::Duration::MAX),
        }).expect("Failed polling device");

        let data = buffer_slice.get_mapped_range();
        let mut pixels = Vec::with_capacity((self.config.image_width * self.config.image_height * 4) as usize);

        for chunk in data.chunks(padded_bytes_per_row as usize) {
            pixels.extend_from_slice(&chunk[..bytes_per_row as usize]);
        }

        let img = image::ImageBuffer::<image::Rgba<u8>, _>::from_raw(
            self.config.image_width,
            self.config.image_height,
            pixels,
        ).expect("Failed to create image buffer");

        drop(data);
        output_buffer.unmap();

        Ok(img)
    }
}
