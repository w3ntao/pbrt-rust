#[derive(Copy, Clone)]
pub struct Point {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Point {
    pub fn new(_x: f32, _y: f32, _z: f32) -> Self {
        return Self {
            x: _x,
            y: _y,
            z: _z,
        };
    }

    pub fn zero() -> Self {
        return Point::new(0.0, 0.0, 0.0);
    }
}
