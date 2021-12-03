use crate::fundamental::point::*;
use crate::fundamental::vector::*;
use crate::ray_tracing::ray::*;

#[derive(Copy, Clone)]
pub struct Intersection {
    pub distance: f32,
    pub ray: *const Ray,
    pub normal: Vector,
}

impl Intersection {
    pub fn new(_distance: f32, _ray: &Ray, _normal: Vector) -> Self {
        return Self {
            distance: _distance,
            ray: _ray,
            normal: _normal,
        };
    }

    pub fn failure() -> Self {
        return Self {
            ray: &Ray::new(Point::zero(), Vector::zero()),
            distance: f32::INFINITY,
            normal: Vector::zero(),
        };
    }

    pub fn intersected(&self) -> bool {
        return self.distance.is_finite();
    }
}
