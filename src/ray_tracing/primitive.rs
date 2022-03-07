use crate::ray_tracing::bounding_box::BoundingBox;
use crate::ray_tracing::intersection::*;
use crate::ray_tracing::ray::*;

pub trait Primitive: Send + Sync {
    fn intersect(&self, ray: &Ray, t_max: f32) -> Intersection;
    fn get_bounds(&self) -> BoundingBox;
}
