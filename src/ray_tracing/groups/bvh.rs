use crate::ray_tracing::ray::*;
use crate::ray_tracing::intersection::*;
use crate::ray_tracing::primitive::Primitive;
use crate::ray_tracing::group::Group;

#[derive(Default)]
pub struct BVH<'a> {
    primitives: Vec<&'a dyn Primitive>,
}

impl<'a> BVH<'a> {
    pub fn new() -> Self { Default::default() }
}

impl<'a> Group<'a> for BVH<'a> {
    fn add(&mut self, p: &'a dyn Primitive) {
        self.primitives.push(p);
    }

    fn intersect(&self, ray: &Ray, previous_distance: f32) -> Intersection {
        return Intersection::failure();
    }
}

impl<'a> BVH<'a> {
    fn build_index(&mut self) {

    }
}
