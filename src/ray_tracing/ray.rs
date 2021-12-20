use crate::fundamental::point::*;
use crate::fundamental::vector::*;

#[derive(Copy, Clone)]
pub struct Ray {
    pub origin: Point,
    pub direction: Vector,
}

impl Ray {
    pub fn new(_origin: Point, _direction: Vector) -> Self {
        return Self {
            origin: _origin,
            direction: _direction,
        };
    }

    pub fn dummy() -> Self {
        Self {
            origin: Point::zero(),
            direction: Vector::zero(),
        }
    }

    pub fn get_point(&self, distance: f32) -> Point {
        return self.origin + distance * self.direction;
    }
}
