use crate::fundamental::point::*;
use crate::fundamental::vector3::*;

#[derive(Copy, Clone)]
pub struct Ray {
    pub origin: Point,
    pub direction: Vector3,
}

impl Ray {
    pub fn new(_origin: Point, _direction: Vector3) -> Self {
        return Self {
            origin: _origin,
            direction: _direction,
        };
    }

    pub fn dummy() -> Self {
        Self {
            origin: Point::invalid(),
            direction: Vector3::invalid(),
        }
    }

    pub fn get_point(&self, distance: f32) -> Point {
        return self.origin + distance * self.direction;
    }
}
