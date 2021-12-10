use crate::ray_tracing::ray::*;
use crate::ray_tracing::intersection::*;
use crate::ray_tracing::bounding_box::BoundingBox;

pub trait Primitive: Send + Sync {
    fn intersect(&self, ray: &Ray, previous_distance: f32) -> Intersection;

    fn get_bounds(&self) -> BoundingBox;
}
