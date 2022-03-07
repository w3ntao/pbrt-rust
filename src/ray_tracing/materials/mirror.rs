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

impl Material for Mirror {
    fn scatter(&self, scattered_ray: &mut Ray, incoming_ray: &Ray, intersection: &Intersection) -> Color {
        scattered_ray.origin = intersection.ray.get_point(intersection.distance);
        scattered_ray.direction = incoming_ray.direction.reflect(intersection.normal);
        return Color::new(1.0, 1.0, 1.0);
        // TODO: I am implementing perfect glass for the time being
        // TODO: that reflects everything
    }
}
