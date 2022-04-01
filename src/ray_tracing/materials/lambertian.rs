use crate::fundamental::color::*;
use crate::fundamental::vector3::*;
use crate::ray_tracing::intersection::*;
use crate::ray_tracing::material::Material;
use crate::ray_tracing::ray::*;

pub struct Lambertian {
    albedo: Color,
}

impl Lambertian {
    pub fn new(_albedo: Color) -> Lambertian {
        Lambertian { albedo: _albedo }
    }
}

impl Material for Lambertian {
    fn scatter(&self, _: &Ray, intersection: &Intersection, scattered_ray: &mut Ray) -> Color {
        let scattered_direction = random_vector_in_hemisphere(intersection.normal);

        scattered_ray.origin = intersection.hit_point;
        scattered_ray.direction = scattered_direction;
        return self.albedo;
    }
}
