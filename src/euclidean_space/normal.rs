use crate::pbrt::*;

#[derive(Copy, Clone, PartialEq)]
pub struct Normal3f {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Normal3f {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        return Self { x, y, z };
    }

    pub fn nan() -> Self {
        return Self {
            x: f64::NAN,
            y: f64::NAN,
            z: f64::NAN,
        };
    }

    pub fn is_non_zero(&self) -> bool {
        return !(self.x == 0.0 && self.y == 0.0 && self.z == 0.0);
    }

    pub fn is_valid(&self) -> bool {
        return self.x.is_finite() && self.y.is_finite() && self.z.is_finite();
    }

    pub fn normalize(&self) -> Normal3f {
        let length = (sqr(self.x) + sqr(self.y) + sqr(self.z)).sqrt();
        return Normal3f {
            x: self.x / length,
            y: self.y / length,
            z: self.z / length,
        };
    }

    pub fn dot(&self, v: Vector3f) -> f64 {
        return self.x * v.x + self.y * v.y + self.z * v.z;
    }

    pub fn abs_dot(&self, v: Vector3f) -> f64 {
        return self.dot(v).abs();
    }

    pub fn face_forward(&self, v: Vector3f) -> Normal3f {
        return if self.dot(v) >= 0.0 { *self } else { -*self };
    }
}

impl Default for Normal3f {
    fn default() -> Self {
        return Self {
            x: f64::NAN,
            y: f64::NAN,
            z: f64::NAN,
        };
    }
}

impl From<Vector3f> for Normal3f {
    fn from(v: Vector3f) -> Self {
        return Self {
            x: v.x,
            y: v.y,
            z: v.z,
        };
    }
}

impl Display for Normal3f {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "[ {}, {}, {} ]", self.x, self.y, self.z)
    }
}

impl Neg for Normal3f {
    type Output = Normal3f;

    fn neg(self) -> Self::Output {
        return Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        };
    }
}

impl Add<Normal3f> for Normal3f {
    type Output = Normal3f;

    fn add(self, rhs: Normal3f) -> Self::Output {
        return Normal3f {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        };
    }
}

impl Mul<f64> for Normal3f {
    type Output = Normal3f;

    fn mul(self, rhs: f64) -> Self::Output {
        return Self {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        };
    }
}

impl Mul<Normal3f> for f64 {
    type Output = Normal3f;

    fn mul(self, rhs: Normal3f) -> Self::Output {
        return Normal3f {
            x: self * rhs.x,
            y: self * rhs.y,
            z: self * rhs.z,
        };
    }
}

impl MulAssign<f64> for Normal3f {
    fn mul_assign(&mut self, rhs: f64) {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
    }
}
