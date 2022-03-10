use crate::fundamental::color::*;
use crate::ray_tracing::intersection::*;
use crate::ray_tracing::material::Material;
use crate::ray_tracing::ray::*;

pub struct NullMaterial {}

impl Material for NullMaterial {
    fn scatter(&self, incoming_ray: &Ray, intersection: &Intersection, scattered_ray: &mut Ray) -> Color {
        panic!("You should never invoke `scatter` from Null");
    }
}
