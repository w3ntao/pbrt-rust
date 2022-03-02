use crate::fundamental::color::*;
use crate::ray_tracing::intersection::*;
use crate::ray_tracing::material::Material;
use crate::ray_tracing::ray::*;

pub struct NullMaterial {}

impl Material for NullMaterial {
    fn scatter(&self, scattered_ray: &mut Ray, incoming_ray: &Ray, intersection: &Intersection) -> Color {
        panic!("You should never invoke `scatter` from Null");
    }
}
