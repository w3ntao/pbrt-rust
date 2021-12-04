use crate::fundamental::point::*;
use crate::fundamental::vector::*;
use crate::ray_tracing::ray::Ray;

#[derive(Copy, Clone)]
pub struct BoundingBox {
    pub min: Point,
    pub max: Point,
}

impl BoundingBox {
    fn new(_min: Point, _max: Point) -> Self {
        return BoundingBox {
            min: _min,
            max: _max,
        };
    }
    
    fn intersect(&self, ray: &Ray) -> (f32, f32) {
        return (f32::NAN, f32::NAN);
    }
}
