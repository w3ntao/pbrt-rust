use rand::thread_rng;
use rand_distr::{Distribution, Normal, NormalError};

use crate::fundamental::rgb_color::*;
use crate::fundamental::vector::*;
use crate::ray_tracing::intersection::*;
use crate::ray_tracing::material::Material;
use crate::ray_tracing::ray::*;

pub struct Mirror {}

impl Mirror {
    pub fn new() -> Mirror {
        Mirror {}
    }
}

fn reflect(vec_in: &Vector, normal: &Vector) -> Vector {
    return *vec_in - 2.0 * dot(vec_in, normal) * (*normal);
}

impl Material for Mirror {
    fn scatter(&self, scattered_ray: &mut Ray, incoming_ray: &Ray, intersection: &Intersection) -> RGBColor {
        scattered_ray.origin = intersection.ray.get_point(intersection.distance) + 0.001 * intersection.normal;
        scattered_ray.direction = reflect(&incoming_ray.direction, &intersection.normal);
        return RGBColor::new(1.0, 1.0, 1.0);
        // TODO: I am implementing perfect glass for the time being
        // TODO: that reflects everything
    }
}
