use crate::ray_tracing::ray::*;
use crate::ray_tracing::intersection::*;
use crate::ray_tracing::primitive_trait::Primitive;
use crate::ray_tracing::group::group_trait::GroupTrait;

#[derive(Default)]
pub struct SimpleGroup<'a> {
    primitives: Vec<&'a (dyn Primitive + 'a)>,
}

impl<'a> SimpleGroup<'a> {
    pub fn new() -> Self { Default::default() }
}

impl<'a> GroupTrait<'a> for SimpleGroup<'a> {
    fn add(&mut self, p: &'a (dyn Primitive + 'a)) {
        self.primitives.push(p);
    }

    fn intersect(&self, ray: &Ray, previous_distance: f32) -> Intersection {
        let mut closest_intersect = Intersection::failure();
        let mut closest_distance = previous_distance;

        for p in &self.primitives {
            let intersect = p.intersect(ray, closest_distance);
            if intersect.intersected() {
                closest_intersect = intersect;
                closest_distance = intersect.distance;
            }
        }
        return closest_intersect;
    }
}
