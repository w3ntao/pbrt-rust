use std::ops;

#[derive(Default, Copy, Clone)]
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
        return Default::default();
    }

    pub fn length_squared(self) -> f32 {
        return dot(self, self);
    }

    pub fn length(self) -> f32 {
        return self.length_squared().sqrt();
    }

    pub fn normalize(self) -> Vector {
        return self / self.length();
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

impl ops::Sub<Vector> for Vector {
    type Output = Vector;
    fn sub(self, rhs: Vector) -> Vector {
        return Vector {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        };
    }
}

impl ops::Mul<Vector> for Vector {
    type Output = Vector;
    fn mul(self, rhs: Vector) -> Vector {
        return Vector {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
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
        assert_ne!(rhs, 0.0);
        return self * (1.0 / rhs);
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

impl ops::DivAssign<f32> for Vector {
    fn div_assign(&mut self, rhs: f32) {
        *self = *self / rhs;
    }
}

pub fn dot(a: Vector, b: Vector) -> f32 {
    return a.x * b.x + a.y * b.y + a.z * b.z;
}

pub fn cross(a: Vector, b: Vector) -> Vector {
    return Vector {
        x: a.y * b.z - a.z * b.y,
        y: a.z * b.x - a.x * b.z,
        z: a.x * b.y - a.y * b.x,
    };
}

pub type Color = Vector;
