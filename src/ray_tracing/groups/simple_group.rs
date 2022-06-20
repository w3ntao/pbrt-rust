use std::sync::Arc;

use crate::fundamental::point::Point;
use crate::ray_tracing::bounding_box::BoundingBox;
use crate::ray_tracing::intersection::*;
use crate::ray_tracing::material::Material;
use crate::ray_tracing::primitive::Primitive;
use crate::ray_tracing::ray::*;

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
    fn intersect(&self, ray: &Ray, t_min: f32, t_max: f32) -> Intersection {
        let mut closest_intersect = Intersection::failure();
        let mut closest_distance_so_far = t_max;

        for p in &self.primitives {
            let intersect = p.intersect(ray, t_min, closest_distance_so_far);
            if intersect.intersected() {
                closest_distance_so_far = intersect.distance;
                closest_intersect = intersect;
            }
        }
        return closest_intersect;
    }

    fn get_bounds(&self) -> BoundingBox {
        return BoundingBox::default();
    }

    fn set_material(&mut self, _: Arc<dyn Material>) {
        panic!("You shouldn't invoke function `set_material)_` from simple_group")
    }

    fn sample(&self) -> Point {
        panic!("sample() is not implemented for SimpleGroup");
    }
}
