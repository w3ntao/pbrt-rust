use crate::vector::*;
use crate::ray::*;

#[derive(Copy, Clone)]
pub struct Intersection {
    pub ray: Ray,
    pub distance: f32,
    pub normal: Vector,
}

impl Intersection {
    pub fn new(_ray: Ray, _distance: f32, _normal: Vector) -> Self {
        return Self {
            ray: _ray,
            distance: _distance,
            normal: _normal,
        };
    }
    
    pub fn failure() -> Self {
        return Self {
            ray: Ray::new(Vector::zero(), Vector::zero()),
            distance: f32::INFINITY,
            normal: Vector::zero(),
        };
    }
}
