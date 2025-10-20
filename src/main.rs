mod camera;
mod color;
mod common;
mod hit;
mod material;
mod ray;
mod raytracer;
mod sphere;
mod vec3;

use std::io;
use std::rc::Rc;
use rand::Rng;
use winit::{event_loop::EventLoop, window::Window};

use camera::Camera;
use color::Color;
use hit::HittableList;
use material::{lambertian, metal, dielectric, diffuse_light};
use ray::Ray;
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

    let world = HittableList::new()
        .add(Box::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0, Rc::new(material_ground))))
        .add(Box::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5, Rc::new(material_diffuse))))
        .add(Box::new(Sphere::new(Point3::new(-1.0, 0.0, -1.0), 0.5, Rc::new(material_glass.clone()))))
        .add(Box::new(Sphere::new(Point3::new(-1.0, 0.0, -1.0), -0.45, Rc::new(material_glass))))
        .add(Box::new(Sphere::new(Point3::new(1.0, 0.0, -1.0), 0.5, Rc::new(material_metal))));

    let camera = Camera::new(
        Point3::new(-2.0, 2.0, 1.0),
        Point3::new(0.0, 0.0, -1.0),
        Vec3::new(0.0, 1.0, 0.0),
        FOV,
        ASPECT_RATIO,
    );

    print!("P3\n{} {}\n255\n", IMAGE_WIDTH, IMAGE_HEIGHT);

    for j in (0..IMAGE_HEIGHT).rev() {
        eprint!("\rScanlines remaining: {}", j);

        for i in 0..IMAGE_WIDTH {
            let mut pixel_color = Color::new(0.0, 0.0, 0.0);

            for _ in 0..SAMPLES_PER_PIXEL {
                let u = (i as f64 + rand::rng().random::<f64>()) / (IMAGE_WIDTH - 1) as f64;
                let v = (j as f64 + rand::rng().random::<f64>()) / (IMAGE_HEIGHT - 1) as f64;
                let r = camera.get_ray(u, v);

                pixel_color += ray_color(&r, &world, MAX_DEPTH);
            }

            color::write_color(&mut io::stdout(), pixel_color, SAMPLES_PER_PIXEL);
        }
    }

    eprintln!("\nDone!");
}
