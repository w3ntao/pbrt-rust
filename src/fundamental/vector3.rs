use std::ops;

use rand::distributions::Uniform;
use rand::prelude::Distribution;
use rand::thread_rng;

use crate::fundamental::point::*;

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

    pub fn zero() -> Self {
        return Vector3::new(0.0, 0.0, 0.0);
    }

    pub fn length_squared(&self) -> f32 {
        return dot(*self, *self);
    }

    pub fn length(&self) -> f32 {
        return self.length_squared().sqrt();
    }

    pub fn normalize(&self) -> Vector3 {
        return *self / self.length();
    }

    pub fn reflect(&self, normal: Vector3) -> Vector3 {
        return *self - 2.0 * dot(*self, normal) * normal;
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
        return Vector3 {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
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

pub fn dot(a: Vector3, b: Vector3) -> f32 {
    return a.x * b.x + a.y * b.y + a.z * b.z;
}

pub fn cross(a: Vector3, b: Vector3) -> Vector3 {
    return Vector3 {
        x: a.y * b.z - a.z * b.y,
        y: a.z * b.x - a.x * b.z,
        z: a.x * b.y - a.y * b.x,
    };
}

pub fn cosine(a: Vector3, b: Vector3) -> f32 {
    return dot(a.normalize(), b.normalize());
}

pub fn random_in_unit_sphere() -> Vector3 {
    // TODO: this is inefficient
    let mut rng = thread_rng();
    let uniform_distribution = Uniform::new(-1.0, 1.0);

    loop {
        let x = uniform_distribution.sample(&mut rng);
        let y = uniform_distribution.sample(&mut rng);
        let z = uniform_distribution.sample(&mut rng);

        let acc = x * x + y * y + z * z;
        if acc > 1.0 || acc < 0.0001 {
            continue;
        }
        return Vector3::new(x, y, z);
    }
}

pub fn random_vector_in_hemisphere(normal: Vector3) -> Vector3 {
    let random_vec = random_in_unit_sphere();

    return {
        if dot(random_vec, normal) < 0.0 {
            -random_vec
        } else {
            random_vec
        }
    };
}
