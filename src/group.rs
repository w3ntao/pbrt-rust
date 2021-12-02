use crate::vector::*;
use crate::ray::*;
use crate::triangle::*;
use crate::intersection::*;

#[derive(Default)]
pub struct Group {
    primitives: Vec<Triangle>,
}

impl Group {
    pub fn new() -> Self { Default::default() }

    pub fn intersect(&self, ray: &Ray, previous_distance: f32) -> Intersection {
        let mut intersect = Intersection::failure();
        let mut closest_distance = previous_distance;

        for p in &self.primitives {
            let temp = p.intersect(ray, closest_distance);
            if temp.hit() {
                intersect = temp;
                closest_distance = temp.distance;
            }
        }
        return intersect;
    }
}
