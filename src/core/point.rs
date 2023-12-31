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

    pub fn check(self) {
        if self.x.is_finite() && self.y.is_finite() && self.z.is_finite() {
            return;
        }
        panic!("illegal Point: {}", self);
    }

    pub fn permute(self, dim_x: usize, dim_y: usize, dim_z: usize) -> Point {
        return Point::new(self[dim_x], self[dim_y], self[dim_z]);
    }

    fn min(&self, b: Point) -> Point {
        return Point::new(self.x.min(b.x), self.y.min(b.y), self.z.min(b.z));
    }

    fn max(&self, b: Point) -> Point {
        return Point::new(self.x.max(b.x), self.y.max(b.y), self.z.max(b.z));
    }
}

impl std::fmt::Display for Point {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}

impl From<Vector3> for Point {
    fn from(v: Vector3) -> Self {
        Point {
            x: v.x,
            y: v.y,
            z: v.z,
        }
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

impl ops::Div<f32> for Point {
    type Output = Point;
    fn div(self, divisor: f32) -> Self::Output {
        return self * (1.0 / divisor);
    }
}

impl ops::Add<Vector3> for Point {
    type Output = Point;
    fn add(self, rhs: Vector3) -> Point {
        return Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        };
    }
}

impl ops::Sub<Vector3> for Point {
    type Output = Point;

    fn sub(self, rhs: Vector3) -> Self::Output {
        return Point {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        };
    }
}

impl ops::AddAssign<Vector3> for Point {
    fn add_assign(&mut self, rhs: Vector3) {
        *self = *self + rhs;
    }
}

impl ops::MulAssign<f32> for Point {
    fn mul_assign(&mut self, rhs: f32) {
        *self = *self * rhs;
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
