use crate::core::bounding_box::BoundingBox;
use crate::core::intersection::*;
use crate::core::material::Material;
use crate::core::ray::*;
use crate::fundamental::point::Point;
use crate::fundamental::utility::Vector3;
use std::sync::Arc;

pub trait Primitive: Send + Sync {
    fn intersect(&self, ray: &Ray, t_min: f32, t_max: f32) -> Intersection;

    fn get_bounds(&self) -> BoundingBox;

    fn set_material(&mut self, material: Arc<dyn Material>);

    fn sample(&self) -> (Point, Vector3) {
        panic!("sample() not implemented for this Primitive");
    }

    fn get_id(&self) -> u128 {
        panic!("get_id() not implemented for this Primitive");
    }

    fn get_area(&self) -> f32 {
        panic!("get_area() not implemented for this Primitive");
    }
}
