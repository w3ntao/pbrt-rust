use crate::ray_tracing::ray::*;
use crate::ray_tracing::intersection::*;

pub trait Primitive {
    fn intersect(&self, ray: &Ray, previous_distance: f32) -> Intersection;
}
