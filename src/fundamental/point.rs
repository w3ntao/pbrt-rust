use std::ops;

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

impl ops::Index<usize> for Point {
    type Output = f32;
    fn index(&self, index: usize) -> &f32 {
        return match index {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => {
                panic!("Point: illegal index: {}", index);
            }
        };
    }
}

impl ops::IndexMut<usize> for Point {
    fn index_mut(&mut self, index: usize) -> &mut f32 {
        return match index {
            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
            _ => {
                panic!("Point: illegal index: {}", index);
            }
        };
    }
}

pub fn max_point(a: Point, b: Point) -> Point {
    return Point::new(
        a.x.max(b.x),
        a.y.max(b.y),
        a.z.max(b.z),
    );
}

pub fn min_point(a: Point, b: Point) -> Point {
    return Point::new(
        a.x.min(b.x),
        a.y.min(b.y),
        a.z.min(b.z),
    );
}
