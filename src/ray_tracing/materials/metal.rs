use crate::fundamental::color::*;
use crate::fundamental::vector3::*;
use crate::fundamental::vector3::random_in_unit_sphere;
use crate::ray_tracing::intersection::*;
use crate::ray_tracing::material::Material;
use crate::ray_tracing::ray::*;

pub struct Metal {
    albedo: Color,
    fuzz: f32,
}

impl Metal {
    pub fn new(_albedo: Color, _fuzz: f32) -> Metal {
        Metal {
            albedo: _albedo,
            fuzz: _fuzz,
        }
    }
}

impl Material for Metal {
    fn scatter(&self, incoming_ray: Ray, intersection: &Intersection) -> (bool, Ray, Color) {
        let reflected = incoming_ray.direction.reflect(intersection.normal);

        let scattered_ray = Ray::new(intersection.hit_point, reflected + self.fuzz * random_in_unit_sphere());

        return (dot(scattered_ray.direction, intersection.normal) > 0.0, scattered_ray, self.albedo);
        // for those light go beneath the surface, consider them not scattered
    }

    fn is_specular(&self) -> bool { return true; }
}
