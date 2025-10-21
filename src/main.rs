mod camera;
mod common;
mod material;
mod raytracer;
mod sphere;
mod vec3;

use futures::executor::block_on;

use camera::Camera;
use material::{lambertian, metal, dielectric, diffuse_light};
use raytracer::RayTracer;
use raytracer::simple::SimpleRayTracer;
use sphere::Sphere;
use vec3::{Point3, Vec3};

fn main() {
    const FOV: f32 = 20.0;
    const ASPECT_RATIO: f32 = 16.0 / 9.0;
    const IMAGE_WIDTH: u32 = 400;
    const IMAGE_HEIGHT: u32 = (IMAGE_WIDTH as f32 / ASPECT_RATIO) as u32;

    let material_ground = lambertian(Vec3::new(0.8, 0.8, 0.0));
    let material_glass = dielectric(1.5);
    let material_diffuse = diffuse_light(Vec3::new(0.9, 0.9, 0.1), 1.5);
    let material_metal = metal(Vec3::new(0.5, 0.5, 0.5), 0.0);

    let mut objects = Vec::new();
    objects.push(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0, material_ground));
    objects.push(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5, material_diffuse));
    objects.push(Sphere::new(Point3::new(-1.0, 0.0, -1.0), 0.5, material_glass.clone()));
    objects.push(Sphere::new(Point3::new(-1.0, 0.0, -1.0), -0.45, material_glass));
    objects.push(Sphere::new(Point3::new(1.0, 0.0, -1.0), 0.5, material_metal));

    let camera = Camera::new(
        Point3::new(-2.0, 2.0, 1.0),
        Point3::new(0.0, 0.0, -1.0),
        Vec3::new(0.0, 1.0, 0.0),
        FOV,
        ASPECT_RATIO,
    );

    let ray_tracer = block_on(SimpleRayTracer::new(
        IMAGE_WIDTH,
        IMAGE_HEIGHT,
        objects,
        &camera,
    ));

    match ray_tracer.render() {
        Err(err) => {
            eprintln!("Error while rendering: {err}");
        },
        Ok(img) => {
            img.save("output.png").expect("Failed to save image");
        },
    }
}
