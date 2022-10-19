use crate::core::pbrt::*;

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

    pub fn invalid() -> Self {
        return Point::new(f32::NAN, f32::NAN, f32::NAN);
    }

    pub fn is_valid(&self) -> bool {
        return !self.x.is_nan() && !self.y.is_nan() && !self.z.is_nan();
    }

    fn min(&self, b: Point) -> Point {
        return Point::new(self.x.min(b.x), self.y.min(b.y), self.z.min(b.z));
    }

    fn max(&self, b: Point) -> Point {
        return Point::new(self.x.max(b.x), self.y.max(b.y), self.z.max(b.z));
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

impl ops::Add<Point> for Point {
    type Output = Point;
    fn add(self, rhs: Point) -> Point {
        return Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        };
    }
}

impl ops::Mul<f32> for Point {
    type Output = Point;
    fn mul(self, scalar: f32) -> Self::Output {
        Point {
            x: self.x * scalar,
            y: self.y * scalar,
            z: self.z * scalar,
        }
    }
}

impl ops::Mul<Point> for f32 {
    type Output = Point;
    fn mul(self, p: Point) -> Self::Output {
        p * self
    }
}

pub fn max_of(points: &[Point]) -> Point {
    let mut _max = points[0];

    for idx in 1..points.len() {
        _max = _max.max(points[idx]);
    }

    return _max;
}

pub fn min_of(points: &[Point]) -> Point {
    let mut _min = points[0];

    for idx in 1..points.len() {
        _min = _min.min(points[idx]);
    }

    return _min;
}
