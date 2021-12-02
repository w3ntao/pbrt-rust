use crate::vector::*;
use crate::ray::*;

#[derive(Copy, Clone)]
pub struct Intersection {
    pub distance: f32,
    pub ray: Ray,
    pub normal: Vector,
}

impl Intersection {
    pub fn new(_distance: f32, _ray: &Ray, _normal: Vector) -> Self {
        return Self {
            distance: _distance,
            ray: *_ray,
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

    pub fn hit(&self) -> bool {
        return self.distance.is_finite();
    }
}
