use std::ops;
use crate::fundamental::point::*;

#[derive(Copy, Clone)]
pub struct Vector {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vector {
    pub fn new(_x: f32, _y: f32, _z: f32) -> Self {
        return Self {
            x: _x,
            y: _y,
            z: _z,
        };
    }

    pub fn zero() -> Self {
        return Vector::new(0.0, 0.0, 0.0);
    }

    pub fn length_squared(&self) -> f32 {
        return dot(self, self);
    }

    pub fn length(&self) -> f32 {
        return self.length_squared().sqrt();
    }

    pub fn normalize(&self) -> Vector {
        return *self / self.length();
    }
}

impl ops::Index<usize> for Vector {
    type Output = f32;
    fn index(&self, index: usize) -> &f32 {
        return match index {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => {
                panic!("Vector: illegal index: {}", index);
            }
        };
    }
}

impl ops::IndexMut<usize> for Vector {
    fn index_mut(&mut self, index: usize) -> &mut f32 {
        return match index {
            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
            _ => {
                panic!("Vector: illegal index: {}", index);
            }
        };
    }
}

impl ops::Neg for Vector {
    type Output = Vector;
    fn neg(self) -> Vector {
        return Vector {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        };
    }
}

impl ops::Add<Vector> for Vector {
    type Output = Vector;
    fn add(self, rhs: Vector) -> Vector {
        return Vector {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        };
    }
}

impl ops::Add<Vector> for Point {
    type Output = Point;
    fn add(self, rhs: Vector) -> Point {
        return Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        };
    }
}

impl ops::Sub<Point> for Point {
    type Output = Vector;
    fn sub(self, rhs: Point) -> Vector {
        return Vector {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        };
    }
}

impl ops::Mul<f32> for Vector {
    type Output = Vector;
    fn mul(self, rhs: f32) -> Vector {
        return Vector {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        };
    }
}

impl ops::Mul<Vector> for f32 {
    type Output = Vector;
    fn mul(self, rhs: Vector) -> Vector {
        return rhs * self;
    }
}

impl ops::Div<f32> for Vector {
    type Output = Vector;
    fn div(self, rhs: f32) -> Vector {
        return Vector {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        };
    }
}

impl ops::AddAssign<Vector> for Vector {
    fn add_assign(&mut self, rhs: Vector) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl ops::MulAssign<Vector> for Vector {
    fn mul_assign(&mut self, rhs: Vector) {
        self.x *= rhs.x;
        self.y *= rhs.y;
        self.z *= rhs.z;
    }
}

pub fn dot(a: &Vector, b: &Vector) -> f32 {
    return a.x * b.x + a.y * b.y + a.z * b.z;
}

pub fn cross(a: Vector, b: Vector) -> Vector {
    return Vector {
        x: a.y * b.z - a.z * b.y,
        y: a.z * b.x - a.x * b.z,
        z: a.x * b.y - a.y * b.x,
    };
}
