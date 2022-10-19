use crate::fundamental::point::*;
use crate::fundamental::vector3::*;

#[derive(Copy, Clone)]
pub struct Ray {
    pub o: Point,
    pub d: Vector3,
}

impl Ray {
    pub fn new(_o: Point, _d: Vector3) -> Self {
        return Self { o: _o, d: _d };
    }

    pub fn get_point(&self, distance: f32) -> Point {
        return self.o + distance * self.d;
    }
}
