use crate::ray::*;
use crate::intersection::*;
use crate::primitive::Primitive;

#[derive(Default)]
pub struct Group {
    primitives: Vec<Box<dyn Primitive>>,
}

impl Group {
    pub fn new() -> Self { Default::default() }

    pub fn add(&mut self, triangle: Box<dyn Primitive>) {
        self.primitives.push(triangle);
    }

    pub fn intersect(&self, ray: &Ray, previous_distance: f32) -> Intersection {
        let mut intersect = Intersection::failure();
        let mut closest_distance = previous_distance;

        for p in &self.primitives {
            let temp = p.intersect(ray, closest_distance);
            if temp.intersected() {
                intersect = temp;
                closest_distance = temp.distance;
            }
        }
        return intersect;
    }
}
