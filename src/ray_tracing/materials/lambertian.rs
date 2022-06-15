use std::sync::Arc;

use rand::random;

use crate::fundamental::color::*;
use crate::fundamental::orthonormal_basis::OrthonormalBasis;
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

fn random_cosine_direction() -> Vector3 {
    let sin2_theta = random_zero_to_one();
    let cos2_theta = 1.0 - sin2_theta;

    let phi = random_in_range(0.0, 2.0 * PI);
    let sin_phi = phi.sin();
    let cos_phi = phi.cos();

    let sin_theta = sin2_theta.sqrt();

    return Vector3::new(sin_phi * sin_theta, cos_phi * sin_theta, cos2_theta.sqrt());
}

impl Material for Lambertian {
    fn scatter(&self, _: Ray, intersection: &Intersection) -> (bool, Ray, Color) {
        let uvw = OrthonormalBasis::build_from_w(intersection.normal);
        let random_direction = uvw.local(random_cosine_direction());

        let scattered_ray = Ray::new(intersection.hit_point, random_direction.normalize());

        return (true, scattered_ray, self.albedo.get_color(intersection.u, intersection.v, intersection.hit_point));
    }

    fn emit(&self, _: f32, _: f32, _: Point) -> Color {
        return Color::black();
    }
}
