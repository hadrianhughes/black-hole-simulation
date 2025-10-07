mod camera;
mod color;
mod common;
mod hit;
mod material;
mod ray;
mod sphere;
mod vec3;

use std::io;
use std::rc::Rc;
use rand::Rng;

use camera::Camera;
use color::Color;
use hit::{Hittable, HittableList};
use material::{Dielectric, Lambertian, Metal};
use ray::Ray;
use sphere::Sphere;
use vec3::Point3;

fn ray_color(ray: &Ray, world: &dyn Hittable, depth: i32) -> Color {
    if depth <= 0 {
        return Color::new(0.0, 0.0, 0.0);
    }

    if let Some(hit) = world.hit_scan(ray, 0.001, std::f64::INFINITY) {
        if let Some(scatter) = hit
            .material
            .as_ref()
            .unwrap()
            .scatter(ray, &hit)
        {
            return scatter.attenuation * ray_color(&scatter.ray, world, depth - 1);
        }

        return Color::new(0.0, 0.0, 0.0);
    }

    let unit_direction = ray.direction().unit();
    let t = 0.5 * (unit_direction.y() + 1.0);
    (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
}

fn main() {
    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const IMAGE_WIDTH: i32 = 400;
    const IMAGE_HEIGHT: i32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as i32;
    const SAMPLES_PER_PIXEL: i32 = 50;
    const MAX_DEPTH: i32 = 50;

    let material_ground = Lambertian::new(Color::new(0.8, 0.8, 0.0));
    let material_left   = Dielectric::new(1.5);
    let material_center = Lambertian::new(Color::new(0.1, 0.2, 0.5));
    let material_right  = Metal::new(Color::new(0.8, 0.6, 0.2), 0.8);

    let world = HittableList::new()
        .add(Box::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0, Rc::new(material_ground))))
        .add(Box::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5, Rc::new(material_center))))
        .add(Box::new(Sphere::new(Point3::new(-1.0, 0.0, -1.0), 0.5, Rc::new(material_left.clone()))))
        .add(Box::new(Sphere::new(Point3::new(-1.0, 0.0, -1.0), -0.4, Rc::new(material_left))))
        .add(Box::new(Sphere::new(Point3::new(1.0, 0.0, -1.0), 0.5, Rc::new(material_right))));

    let camera = Camera::new(ASPECT_RATIO);

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
