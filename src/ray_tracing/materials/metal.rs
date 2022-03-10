use crate::fundamental::color::*;
use crate::fundamental::vector3::*;
use crate::fundamental::vector3::random_in_unit_sphere;
use crate::ray_tracing::intersection::*;
use crate::ray_tracing::material::Material;
use crate::ray_tracing::ray::*;

pub struct Metal {
    pub albedo: Color,
    pub fuzz: f32,
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
    fn scatter(&self, incoming_ray: &Ray, intersection: &Intersection, scattered_ray: &mut Ray) -> Color {
        let reflected = incoming_ray.direction.reflect(intersection.normal);
        scattered_ray.origin = intersection.ray.get_point(intersection.distance);
        scattered_ray.direction = reflected + self.fuzz * random_in_unit_sphere();

        if dot(scattered_ray.direction, intersection.normal) <= 0.0 {
            return Color::black();
        }

        return self.albedo;
    }
}
