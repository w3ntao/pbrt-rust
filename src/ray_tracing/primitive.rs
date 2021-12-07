use std::rc::Rc;
use crate::ray_tracing::ray::*;
use crate::ray_tracing::intersection::*;
use crate::ray_tracing::bounding_box::BoundingBox;

pub trait Primitive {
    fn intersect(&self, ray: Rc<Ray>, previous_distance: f32) -> Intersection;

    fn get_bounds(&self) -> BoundingBox;
}
