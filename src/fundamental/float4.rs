use std::ops;

use crate::fundamental::point::Point;
use crate::fundamental::vector::Vector;

pub struct Float4 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

impl Float4 {
    pub fn new(_x: f32, _y: f32, _z: f32, _w: f32) -> Float4 {
        Float4 {
            x: _x,
            y: _y,
            z: _z,
            w: _w,
        }
    }

    pub fn from_point(p: &Point) -> Float4 {
        Float4 {
            x: p.x,
            y: p.y,
            z: p.z,
            w: 1.0,
        }
    }

    pub fn from_vec3(v: &Vector) -> Float4 {
        Float4 {
            x: v.x,
            y: v.y,
            z: v.z,
            w: 0.0,
        }
    }
}

impl ops::Index<usize> for Float4 {
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

impl ops::IndexMut<usize> for Float4 {
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

impl ops::Add<Float4> for Float4 {
    type Output = Float4;
    fn add(self, rhs: Float4) -> Float4 {
        return Float4 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
            w: self.w + rhs.w,
        };
    }
}

impl ops::Sub<Float4> for Float4 {
    type Output = Float4;
    fn sub(self, rhs: Float4) -> Float4 {
        return Float4 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
            w: self.w - rhs.w,
        };
    }
}

impl ops::Mul<Float4> for Float4 {
    type Output = Float4;
    fn mul(self, rhs: Float4) -> Float4 {
        return Float4 {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
            w: self.w * rhs.w,
        };
    }
}

impl ops::Mul<f32> for Float4 {
    type Output = Float4;
    fn mul(self, scalar: f32) -> Float4 {
        return Float4 {
            x: self.x * scalar,
            y: self.y * scalar,
            z: self.z * scalar,
            w: self.w * scalar,
        };
    }
}

impl ops::Mul<Float4> for f32 {
    type Output = Float4;
    fn mul(self, f4: Float4) -> Float4 {
        return f4 * self;
    }
}

impl ops::Div<Float4> for Float4 {
    type Output = Float4;
    fn div(self, rhs: Float4) -> Float4 {
        return Float4 {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
            z: self.z / rhs.z,
            w: self.w / rhs.w,
        };
    }
}

impl ops::Div<f32> for Float4 {
    type Output = Float4;
    fn div(self, scalar: f32) -> Float4 {
        return Float4 {
            x: self.x / scalar,
            y: self.y / scalar,
            z: self.z / scalar,
            w: self.w / scalar,
        };
    }
}

impl ops::Neg for Float4 {
    type Output = Float4;
    fn neg(self) -> Float4 {
        return Float4 {
            x: -self.x,
            y: -self.y,
            z: -self.z,
            w: -self.w,
        };
    }
}
