use crate::core::pbrt::*;

#[derive(Copy, Clone)]
pub struct Normal {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Normal {
    pub fn new(_x: f32, _y: f32, _z: f32) -> Self {
        return Self {
            x: _x,
            y: _y,
            z: _z,
        };
    }

    pub fn invalid() -> Self {
        return Normal::new(f32::NAN, f32::NAN, f32::NAN);
    }

    pub fn dot(&self, v: Vector3) -> f32 {
        let mut product = 0.0;
        for idx in 0..3 {
            product += self[idx] * v[idx];
        }

        return product;
    }

    pub fn length_squared(&self) -> f32 {
        return self.dot(Vector3::from(*self));
    }

    pub fn length(&self) -> f32 {
        return self.length_squared().sqrt();
    }

    pub fn normalize(&self) -> Normal {
        return *self / self.length();
    }

    pub fn cosine(&self, v: Vector3) -> f32 {
        return self.dot(v) / (self.length() * v.length());
    }

    pub fn face_forward(&self, v: Vector3) -> Normal {
        return if self.dot(v) < 0.0 { -*self } else { *self };
    }
}

impl From<Vector3> for Normal {
    fn from(v: Vector3) -> Self {
        Normal {
            x: v.x,
            y: v.y,
            z: v.z,
        }
    }
}

impl ops::Index<usize> for Normal {
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

impl ops::IndexMut<usize> for Normal {
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

impl ops::Neg for Normal {
    type Output = Normal;
    fn neg(self) -> Normal {
        return Normal {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        };
    }
}

impl ops::Add<Normal> for Normal {
    type Output = Normal;
    fn add(self, n: Normal) -> Normal {
        return Normal {
            x: self.x + n.x,
            y: self.y + n.y,
            z: self.z + n.z,
        };
    }
}

impl ops::Sub<Normal> for Normal {
    type Output = Normal;
    fn sub(self, n: Normal) -> Normal {
        return Normal {
            x: self.x - n.x,
            y: self.y - n.y,
            z: self.z - n.z,
        };
    }
}

impl ops::Mul<f32> for Normal {
    type Output = Normal;
    fn mul(self, factor: f32) -> Normal {
        return Normal {
            x: self.x * factor,
            y: self.y * factor,
            z: self.z * factor,
        };
    }
}

impl ops::Mul<Normal> for f32 {
    type Output = Normal;
    fn mul(self, n: Normal) -> Normal {
        return n * self;
    }
}

impl ops::Div<f32> for Normal {
    type Output = Normal;
    fn div(self, divisor: f32) -> Normal {
        let inv = 1.0 / divisor;
        return Normal {
            x: self.x * inv,
            y: self.y * inv,
            z: self.z * inv,
        };
    }
}

impl ops::AddAssign<Normal> for Normal {
    fn add_assign(&mut self, n: Normal) {
        self.x += n.x;
        self.y += n.y;
        self.z += n.z;
    }
}
