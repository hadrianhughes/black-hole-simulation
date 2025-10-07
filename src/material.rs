use crate::color::Color;
use crate::hit::Hit;
use crate::ray::Ray;
use crate::vec3;

pub struct ScatterResult {
    pub ray: Ray,
    pub attenuation: Color,
}

pub trait Material {
    fn scatter(&self, ray: &Ray, hit: &Hit) -> Option<ScatterResult>;
}

pub struct Lambertian {
    albedo: Color,
}

impl Lambertian {
    pub fn new(albedo: Color) -> Self {
        Lambertian { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(&self, _: &Ray, hit: &Hit) -> Option<ScatterResult> {
        let mut scatter_direction = hit.normal + vec3::random_unit_vector();

        if scatter_direction.is_near_zero() {
            scatter_direction = hit.normal;
        }

        Some(ScatterResult {
            ray: Ray::new(hit.position, scatter_direction),
            attenuation: self.albedo,
        })
    }
}

pub struct Metal {
    albedo: Color,
}

impl Metal {
    pub fn new(albedo: Color) -> Self {
        Metal { albedo }
    }
}

impl Material for Metal {
    fn scatter(&self, ray: &Ray, hit: &Hit) -> Option<ScatterResult> {
        let reflected = ray
            .direction()
            .unit()
            .reflect(hit.normal);

        let scattered_ray = Ray::new(hit.position, reflected);

        let did_scatter = vec3::dot(scattered_ray.direction(), hit.normal) > 0.0;
        if did_scatter {
            Some(ScatterResult {
                ray: scattered_ray,
                attenuation: self.albedo,
            })
        } else {
            None
        }
    }
}
