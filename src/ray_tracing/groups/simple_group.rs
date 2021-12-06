use crate::ray_tracing::bounding_box::BoundingBox;
use crate::ray_tracing::ray::*;
use crate::ray_tracing::intersection::*;
use crate::ray_tracing::primitive::Primitive;
use crate::ray_tracing::group::Group;

#[derive(Default)]
pub struct SimpleGroup<'a> {
    primitives: Vec<&'a dyn Primitive>,
}

impl<'a> SimpleGroup<'a> {
    pub fn new() -> Self { Default::default() }
}

impl<'a> Group<'a> for SimpleGroup<'a> {
    fn add(&mut self, p: &'a dyn Primitive) {
        self.primitives.push(p);
    }
}

impl<'a> Primitive for SimpleGroup<'a> {
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

    fn get_bounds(&self) -> BoundingBox {
        return BoundingBox::default();
    }
}
