use std::ops;

use crate::core::point::Point;
use crate::core::vector3::Vector3;

#[derive(Clone, Copy)]
pub struct Vector4 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

impl Vector4 {
    pub fn new(_x: f32, _y: f32, _z: f32, _w: f32) -> Vector4 {
        Vector4 {
            x: _x,
            y: _y,
            z: _z,
            w: _w,
        }
    }

    pub fn zero() -> Vector4 {
        Vector4 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
            w: 0.0,
        }
    }
}

impl From<Point> for Vector4 {
    fn from(p: Point) -> Self {
        Vector4 {
            x: p.x,
            y: p.y,
            z: p.z,
            w: 1.0,
        }
    }
}

impl From<Vector3> for Vector4 {
    fn from(v3: Vector3) -> Self {
        Vector4 {
            x: v3.x,
            y: v3.y,
            z: v3.z,
            w: 0.0,
        }
    }
}

impl ops::Index<usize> for Vector4 {
    type Output = f32;
    fn index(&self, index: usize) -> &f32 {
        return match index {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            3 => &self.w,
            _ => {
                panic!("Float4: illegal index: {}", index);
            }
        };
    }
}

impl ops::IndexMut<usize> for Vector4 {
    fn index_mut(&mut self, index: usize) -> &mut f32 {
        return match index {
            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
            3 => &mut self.w,
            _ => {
                panic!("Float4: illegal index: {}", index);
            }
        };
    }
}

impl ops::Add<Vector4> for Vector4 {
    type Output = Vector4;
    fn add(self, rhs: Vector4) -> Vector4 {
        return Vector4 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
            w: self.w + rhs.w,
        };
    }
}

impl ops::Sub<Vector4> for Vector4 {
    type Output = Vector4;
    fn sub(self, rhs: Vector4) -> Vector4 {
        return Vector4 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
            w: self.w - rhs.w,
        };
    }
}

impl ops::Mul<Vector4> for Vector4 {
    type Output = Vector4;
    fn mul(self, rhs: Vector4) -> Vector4 {
        return Vector4 {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
            w: self.w * rhs.w,
        };
    }
}

impl ops::Mul<f32> for Vector4 {
    type Output = Vector4;
    fn mul(self, scalar: f32) -> Vector4 {
        return Vector4 {
            x: self.x * scalar,
            y: self.y * scalar,
            z: self.z * scalar,
            w: self.w * scalar,
        };
    }
}

impl ops::Mul<Vector4> for f32 {
    type Output = Vector4;
    fn mul(self, f4: Vector4) -> Vector4 {
        return f4 * self;
    }
}

impl ops::Div<Vector4> for Vector4 {
    type Output = Vector4;
    fn div(self, rhs: Vector4) -> Vector4 {
        return Vector4 {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
            z: self.z / rhs.z,
            w: self.w / rhs.w,
        };
    }
}

impl ops::Div<f32> for Vector4 {
    type Output = Vector4;
    fn div(self, divisor: f32) -> Vector4 {
        return Vector4 {
            x: self.x / divisor,
            y: self.y / divisor,
            z: self.z / divisor,
            w: self.w / divisor,
        };
    }
}

impl ops::Neg for Vector4 {
    type Output = Vector4;
    fn neg(self) -> Vector4 {
        return Vector4 {
            x: -self.x,
            y: -self.y,
            z: -self.z,
            w: -self.w,
        };
    }
}

pub fn dot(a: Vector4, b: Vector4) -> f32 {
    let mut product = 0.0;
    for idx in 0..4 {
        product += a[idx] * b[idx];
    }

    return product;
}

impl From<Vector4> for Vector3 {
    fn from(v4: Vector4) -> Self {
        Vector3 {
            x: v4.x,
            y: v4.y,
            z: v4.z,
        }
    }
}

impl From<Vector4> for Point {
    fn from(v4: Vector4) -> Self {
        Point {
            x: v4.x / v4.w,
            y: v4.y / v4.w,
            z: v4.z / v4.w,
        }
    }
}
