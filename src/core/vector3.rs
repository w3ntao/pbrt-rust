use crate::core::pbrt::*;

#[derive(Copy, Clone)]
pub struct Vector3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vector3 {
    pub fn new(_x: f32, _y: f32, _z: f32) -> Self {
        return Self {
            x: _x,
            y: _y,
            z: _z,
        };
    }

    pub fn invalid() -> Self {
        return Vector3::new(f32::NAN, f32::NAN, f32::NAN);
    }

    pub fn max_dimension(self) -> usize {
        if self.x > self.y && self.x > self.z {
            return 0;
        }
        if self.y > self.z {
            return 1;
        }
        return 2;
    }

    pub fn max_component(self) -> f32 {
        return self.x.max(self.y).max(self.z);
    }

    pub fn abs(self) -> Vector3 {
        return Vector3::new(self.x.abs(), self.y.abs(), self.z.abs());
    }

    pub fn permute(self, dim_x: usize, dim_y: usize, dim_z: usize) -> Vector3 {
        return Vector3::new(self[dim_x], self[dim_y], self[dim_z]);
    }

    pub fn dot(&self, rhs: Vector3) -> f32 {
        let mut product = 0.0;
        for idx in 0..3 {
            product += self[idx] * rhs[idx];
        }

        return product;
    }

    pub fn length_squared(&self) -> f32 {
        return self.dot(*self);
    }

    pub fn length(&self) -> f32 {
        return self.length_squared().sqrt();
    }

    pub fn normalize(&self) -> Vector3 {
        return *self / self.length();
    }

    pub fn cosine(&self, v: Vector3) -> f32 {
        return self.dot(v) / (self.length() * v.length());
    }

    pub fn reflect(&self, normal: Normal) -> Vector3 {
        return *self - 2.0 * normal.dot(*self) * Vector3::from(normal);
    }
}

impl std::fmt::Display for Vector3 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}

impl From<Point> for Vector3 {
    fn from(p: Point) -> Self {
        Vector3 {
            x: p.x,
            y: p.y,
            z: p.z,
        }
    }
}

impl From<Normal> for Vector3 {
    fn from(n: Normal) -> Self {
        Vector3 {
            x: n.x,
            y: n.y,
            z: n.z,
        }
    }
}

impl ops::Index<usize> for Vector3 {
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

impl ops::IndexMut<usize> for Vector3 {
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

impl ops::Neg for Vector3 {
    type Output = Vector3;
    fn neg(self) -> Vector3 {
        return Vector3 {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        };
    }
}

impl ops::Add<Vector3> for Vector3 {
    type Output = Vector3;
    fn add(self, rhs: Vector3) -> Vector3 {
        return Vector3 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        };
    }
}

impl ops::Sub<Vector3> for Vector3 {
    type Output = Vector3;
    fn sub(self, rhs: Vector3) -> Vector3 {
        return Vector3 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        };
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

impl ops::Sub<Point> for Point {
    type Output = Vector3;
    fn sub(self, rhs: Point) -> Vector3 {
        return Vector3 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        };
    }
}

impl ops::Mul<f32> for Vector3 {
    type Output = Vector3;
    fn mul(self, rhs: f32) -> Vector3 {
        return Vector3 {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        };
    }
}

impl ops::Mul<Vector3> for f32 {
    type Output = Vector3;
    fn mul(self, rhs: Vector3) -> Vector3 {
        return rhs * self;
    }
}

impl ops::Div<f32> for Vector3 {
    type Output = Vector3;
    fn div(self, rhs: f32) -> Vector3 {
        let inv = 1.0 / rhs;
        return Vector3 {
            x: self.x * inv,
            y: self.y * inv,
            z: self.z * inv,
        };
    }
}

impl ops::AddAssign<Vector3> for Vector3 {
    fn add_assign(&mut self, rhs: Vector3) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl ops::MulAssign<Vector3> for Vector3 {
    fn mul_assign(&mut self, rhs: Vector3) {
        self.x *= rhs.x;
        self.y *= rhs.y;
        self.z *= rhs.z;
    }
}

pub fn cross(a: Vector3, b: Vector3) -> Vector3 {
    return Vector3 {
        x: a.y * b.z - a.z * b.y,
        y: a.z * b.x - a.x * b.z,
        z: a.x * b.y - a.y * b.x,
    };
}

pub fn random_in_unit_sphere() -> Vector3 {
    let phi = random_f32(0.0, 2.0 * PI);
    let sin_phi = phi.sin();
    let cos_phi = phi.cos();
    let cos_theta = random_f32(-1.0, 1.0);
    let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

    return Vector3::new(sin_phi * sin_theta, cos_phi * sin_theta, cos_theta);
}

pub fn random_vector_in_disk() -> (f32, f32) {
    let r = random_f32(0.0, 1.0).sqrt();
    let theta = random_f32(0.0, 2.0 * PI);

    return (r * theta.sin(), r * theta.cos());
}
