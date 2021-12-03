use crate::ray_tracing::ray::*;
use crate::ray_tracing::intersection::*;
use crate::ray_tracing::primitive_trait::Primitive;

#[derive(Default)]
pub struct Group {
    primitives: Vec<Box<dyn Primitive>>,
}

impl Group {
    pub fn new() -> Self { Default::default() }

    pub fn add(&mut self, primitive: Box<dyn Primitive>) {
        self.primitives.push(primitive);
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
