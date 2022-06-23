use std::sync::Arc;

use crate::fundamental::point::Point;
use crate::fundamental::utility::Vector3;
use crate::ray_tracing::bounding_box::BoundingBox;
use crate::ray_tracing::intersection::*;
use crate::ray_tracing::material::Material;
use crate::ray_tracing::ray::*;

pub trait Primitive: Send + Sync {
    fn intersect(&self, ray: &Ray, t_min: f32, t_max: f32) -> Intersection;
    
    fn get_bounds(&self) -> BoundingBox;
    
    fn set_material(&mut self, material: Arc<dyn Material>);
    
    fn sample(&self) -> (Point, Vector3);
    
    fn get_id(&self) -> u128;

    fn get_area(&self) -> f32;
}
