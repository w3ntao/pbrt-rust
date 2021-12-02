use crate::fundamental::vector::*;

#[derive(Copy, Clone)]
pub struct Ray {
    pub origin: Vector,
    pub direction: Vector,
}

impl Ray {
    pub fn new(_origin: Vector, _direction: Vector) -> Self {
        return Self {
            origin: _origin,
            direction: _direction,
        };
    }

    pub fn get_point(&self, distance: f32) -> Vector {
        return self.origin + distance * self.direction;
    }
}
