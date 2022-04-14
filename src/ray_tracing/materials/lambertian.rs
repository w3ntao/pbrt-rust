use std::sync::Arc;

use crate::fundamental::color::*;
use crate::fundamental::point::Point;
use crate::fundamental::vector3::*;
use crate::ray_tracing::intersection::*;
use crate::ray_tracing::material::Material;
use crate::ray_tracing::ray::*;
use crate::ray_tracing::texture::Texture;

pub struct Lambertian {
    albedo: Arc<dyn Texture>,
}

impl Lambertian {
    pub fn new(texture: Arc<dyn Texture>) -> Lambertian {
        Lambertian { albedo: texture }
    }
}

impl Material for Lambertian {
    fn scatter(&self, _: &Ray, intersection: &Intersection, scattered_ray: &mut Ray) -> Color {
        let scattered_direction = random_vector_in_hemisphere(intersection.normal);

        scattered_ray.origin = intersection.hit_point;
        scattered_ray.direction = scattered_direction;
        return self.albedo.get_color(intersection.u, intersection.v, intersection.hit_point);
    }

    fn emit(&self, _: f32, _: f32, _: Point) -> Color {
        return Color::black();
    }
}
