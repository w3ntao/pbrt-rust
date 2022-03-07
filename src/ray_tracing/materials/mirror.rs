use rand::thread_rng;
use rand_distr::{Distribution, Normal, NormalError};

use crate::fundamental::color::*;
use crate::fundamental::vector3::*;
use crate::ray_tracing::intersection::*;
use crate::ray_tracing::material::Material;
use crate::ray_tracing::ray::*;

pub struct Mirror {}

impl Mirror {
    pub fn new() -> Mirror {
        Mirror {}
    }
}

fn reflect(vec_in: Vector3, normal: Vector3) -> Vector3 {
    return vec_in - 2.0 * dot(vec_in, normal) * normal;
}

impl Material for Mirror {
    fn scatter(&self, scattered_ray: &mut Ray, incoming_ray: &Ray, intersection: &Intersection) -> Color {
        scattered_ray.origin = intersection.ray.get_point(intersection.distance);
        scattered_ray.direction = reflect(incoming_ray.direction, intersection.normal);
        return Color::new(1.0, 1.0, 1.0);
        // TODO: I am implementing perfect glass for the time being
        // TODO: that reflects everything
    }
}
