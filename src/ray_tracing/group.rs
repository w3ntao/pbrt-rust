use crate::ray_tracing::ray::*;
use crate::ray_tracing::intersection::*;
use crate::ray_tracing::primitive_trait::Primitive;

#[derive(Default)]
pub struct Group<'a> {
    primitives: Vec<&'a (dyn Primitive + 'a)>,
}

impl<'a> Group<'a> {
    pub fn new() -> Self { Default::default() }

    pub fn add(&mut self, p: &'a (dyn Primitive + 'a)) {
        self.primitives.push(p);
    }

    pub fn intersect(&self, ray: &Ray, previous_distance: f32) -> Intersection {
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
