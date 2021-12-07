use std::rc::Rc;
use crate::fundamental::point::*;
use crate::fundamental::vector::*;
use crate::ray_tracing::ray::*;

#[derive(Clone)]
pub struct Intersection {
    pub distance: f32,
    pub ray: Rc<Ray>,
    pub normal: Vector,
}

impl Intersection {
    pub fn new(_distance: f32, _ray: Rc<Ray>, _normal: Vector) -> Self {
        return Self {
            distance: _distance,
            ray: _ray,
            normal: _normal,
        };
    }

    pub fn failure() -> Self {
        return Self {
            ray: Rc::new(Ray::new(Point::zero(), Vector::zero())),
            distance: f32::INFINITY,
            normal: Vector::zero(),
        };
    }

    pub fn intersected(&self) -> bool {
        return self.distance.is_finite();
    }
}
