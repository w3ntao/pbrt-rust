use crate::pbrt::*;

#[derive(Copy, Clone)]
pub struct Normal3f {
    pub x: Float,
    pub y: Float,
    pub z: Float,
}

impl Normal3f {
    pub fn new(x: Float, y: Float, z: Float) -> Self {
        return Self { x, y, z };
    }

    pub fn nan() -> Self {
        return Self {
            x: Float::NAN,
            y: Float::NAN,
            z: Float::NAN,
        };
    }

    pub fn normalize(&self) -> Normal3f {
        let length = (sqr(self.x) + sqr(self.y) + sqr(self.z)).sqrt();
        return Normal3f {
            x: self.x / length,
            y: self.y / length,
            z: self.z / length,
        };
    }

    pub fn dot(&self, v: Vector3f) -> Float {
        return self.x * v.x + self.y * v.y + self.z * v.z;
    }

    pub fn face_forward(&self, v: Vector3f) -> Normal3f {
        return if self.dot(v) >= 0.0 { *self } else { -*self };
    }
}

impl Default for Normal3f {
    fn default() -> Self {
        return Self {
            x: Float::NAN,
            y: Float::NAN,
            z: Float::NAN,
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
