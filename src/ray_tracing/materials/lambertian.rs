use std::sync::Arc;

use crate::fundamental::color::*;
use crate::fundamental::point::Point;
use crate::fundamental::utility::*;
use crate::fundamental::vector3::*;
use crate::ray_tracing::intersection::*;
use crate::ray_tracing::material::Material;
use crate::ray_tracing::ray::*;
use crate::ray_tracing::texture::Texture;

struct UniformSampler {}

impl UniformSampler {
    pub fn sample(&self, normal: Vector3) -> (Vector3, f32) {
        //https://www.pbr-book.org/3ed-2018/Monte_Carlo_Integration/2D_Sampling_with_Multidimensional_Transformations#UniformlySamplingaHemisphere
        let phi = random_in_range(0.0, 2.0 * PI);
        let sin_phi = phi.sin();
        let cos_phi = phi.cos();

        let cos_theta = random_in_range(-1.0, 1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let mut scattered_direction = Vector3::new(sin_phi * sin_theta, cos_phi * sin_theta, cos_theta);
        if dot(scattered_direction, normal) < 0.0 {
            scattered_direction = -scattered_direction;
        }

        return (scattered_direction, 1.0);
    }
}

pub struct Lambertian {
    albedo: Arc<dyn Texture>,
    sampler: UniformSampler,
}

impl Lambertian {
    pub fn new(texture: Arc<dyn Texture>) -> Lambertian {
        Lambertian {
            albedo: texture,
            sampler: UniformSampler {},
        }
    }
}

impl Material for Lambertian {
    fn scatter(&self, _: &Ray, intersection: &Intersection, scattered_ray: &mut Ray) -> Color {
        let (scattered_direction, pdf) = self.sampler.sample(intersection.normal);

        scattered_ray.origin = intersection.hit_point;
        scattered_ray.direction = scattered_direction;
        return self.albedo.get_color(intersection.u, intersection.v, intersection.hit_point) / pdf;
    }

    fn emit(&self, _: f32, _: f32, _: Point) -> Color {
        return Color::black();
    }
}
