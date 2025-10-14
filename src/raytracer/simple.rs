use crate::camera::Camera;
use crate::raytracer::RayTracer;
use crate::sphere::Sphere;

pub struct SimpleRayTracer {
    objects: Vec<Box<Sphere>>,
    camera: Camera,
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
}
