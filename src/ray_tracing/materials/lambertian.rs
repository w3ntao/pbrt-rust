use std::sync::Arc;

use rand::random;

use crate::fundamental::color::*;
use crate::fundamental::point::Point;
use crate::fundamental::utility::*;
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
        Lambertian {
            albedo: texture,
        }
    }
}

impl Material for Lambertian {
    fn scatter(&self, _: &Ray, intersection: &Intersection) -> (Ray, Color) {
        //https://www.pbr-book.org/3ed-2018/Monte_Carlo_Integration/2D_Sampling_with_Multidimensional_Transformations#UniformlySamplingaHemisphere

        let random_direction = random_in_unit_sphere();
        let scattered_ray = Ray::new(intersection.hit_point, (intersection.normal.normalize() + random_direction).normalize());

        return (scattered_ray, self.albedo.get_color(intersection.u, intersection.v, intersection.hit_point));
    }

    fn emit(&self, _: f32, _: f32, _: Point) -> Color {
        return Color::black();
    }
}
