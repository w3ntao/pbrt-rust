use crate::ray::*;
use crate::intersection::*;

pub trait Primitive {
    fn intersect(&self, ray: &Ray, previous_distance: f32) -> Intersection;
}
