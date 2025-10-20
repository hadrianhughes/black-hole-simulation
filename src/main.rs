mod camera;
mod color;
mod common;
mod hit;
mod material;
mod ray;
mod raytracer;
mod sphere;
mod vec3;

use winit::{event_loop::EventLoop, window::Window};

use camera::Camera;
use color::Color;
use material::{lambertian, metal, dielectric, diffuse_light};
use ray::Ray;
use raytracer::simple::SimpleRayTracer;
use sphere::Sphere;
use vec3::{Point3, Vec3};

fn main() {
    const FOV: f64 = 20.0;
    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const IMAGE_WIDTH: i32 = 400;
    const IMAGE_HEIGHT: i32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as i32;
    const SAMPLES_PER_PIXEL: i32 = 50;
    const MAX_DEPTH: i32 = 50;

    let event_loop = EventLoop::new().unwrap();
    let window_attributes = Window::default_attributes().with_title("Black Hole Simulation");
    let window = event_loop.create_window(window_attributes).unwrap();

    let material_ground = lambertian(Color::new(0.8, 0.8, 0.0));
    let material_glass = dielectric(1.5);
    let material_diffuse = diffuse_light(Color::new(0.9, 0.9, 0.1), 1.5);
    let material_metal = metal(Color::new(0.5, 0.5, 0.5), 0.0);

    let objects = Vec::new()
        .push(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0, material_ground))
        .push(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5, material_diffuse))
        .push(Sphere::new(Point3::new(-1.0, 0.0, -1.0), 0.5, material_glass.clone()))
        .push(Sphere::new(Point3::new(-1.0, 0.0, -1.0), -0.45, material_glass))
        .push(Sphere::new(Point3::new(1.0, 0.0, -1.0), 0.5, material_metal));

    let camera = Camera::new(
        Point3::new(-2.0, 2.0, 1.0),
        Point3::new(0.0, 0.0, -1.0),
        Vec3::new(0.0, 1.0, 0.0),
        FOV,
        ASPECT_RATIO,
    );

    let ray_tracer = SimpleRayTracer::new(
        IMAGE_WIDTH,
        IMAGE_HEIGHT,
        &window,
        objects,
        &camera,
    );

    ray_tracer.render();
}
