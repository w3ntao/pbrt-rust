use crate::ray_tracing::bounding_box::BoundingBox;
use crate::ray_tracing::ray::*;
use crate::ray_tracing::intersection::*;
use crate::ray_tracing::primitive::Primitive;

pub trait Group<'a> {
    fn add(&mut self, p: &'a dyn Primitive);
}

impl<'a> Primitive for dyn Group<'a> {
    fn intersect(&self, ray: &Ray, previous_distance: f32) -> Intersection {
        return self.intersect(ray, previous_distance);
    }

    fn get_bounds(&self) -> BoundingBox {
        return self.get_bounds();
    }
}
