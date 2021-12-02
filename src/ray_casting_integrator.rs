use crate::vector::*;
use crate::ray::*;


pub struct RayCastingIntegrator {}

impl RayCastingIntegrator {
    pub fn new() -> Self {
        return Self {};
    }

    pub fn get_radiance(&self, ray: Ray) -> Vector {
        // TODO: unfinished
        return Vector::zero();
    }
}
