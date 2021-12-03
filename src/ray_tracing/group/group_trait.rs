use crate::ray_tracing::ray::*;
use crate::ray_tracing::intersection::*;
use crate::ray_tracing::primitive_trait::Primitive;

pub trait GroupTrait<'a> {
    fn add(&mut self, p: &'a dyn Primitive);
    fn intersect(&self, ray: &Ray, previous_distance: f32) -> Intersection;
}

impl<'a> Primitive for dyn GroupTrait<'a> {
     fn intersect(&self, ray: &Ray, previous_distance: f32) -> Intersection {
         return self.intersect(ray, previous_distance);
     }
}
