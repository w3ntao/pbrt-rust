use crate::fundamental::color::*;
use crate::fundamental::point::Point;
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
    fn scatter(&self, incoming_ray: Ray, intersection: &Intersection) -> (Ray, Color) {
        let reflected = incoming_ray.direction.reflect(intersection.normal);

        let scattered_ray = Ray::new(intersection.hit_point, reflected + self.fuzz * random_in_unit_sphere());

        if dot(scattered_ray.direction, intersection.normal) <= 0.0 {
            return (scattered_ray, Color::black());
        }

        return (scattered_ray, self.albedo);
    }

    fn emit(&self, _: f32, _: f32, _: Point) -> Color {
        return Color::black();
    }
}
