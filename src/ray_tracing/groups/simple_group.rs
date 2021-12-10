use std::sync::Arc;
use crate::ray_tracing::bounding_box::BoundingBox;
use crate::ray_tracing::ray::*;
use crate::ray_tracing::intersection::*;
use crate::ray_tracing::primitive::Primitive;

#[derive(Default)]
pub struct SimpleGroup {
    primitives: Vec<Arc<dyn Primitive>>,
}

impl SimpleGroup {
    pub fn new() -> Self { Default::default() }
}

impl SimpleGroup {
    pub(crate) fn add(&mut self, p: Arc<dyn Primitive>) {
        self.primitives.push(p);
    }
}

impl Primitive for SimpleGroup {
        fn intersect(&self, ray: &Ray, previous_distance: f32) -> Intersection {
        let mut closest_intersect = Intersection::failure();
        let mut closest_distance = previous_distance;

        for p in &self.primitives {
            let intersect = p.intersect(ray, closest_distance);
            if intersect.intersected() {
                closest_distance = intersect.distance;
                closest_intersect = intersect;
            }
        }
        return closest_intersect;
    }

    fn get_bounds(&self) -> BoundingBox {
        return BoundingBox::default();
    }
}
