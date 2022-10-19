use crate::core::interfaces::*;
use std::ops;

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

#[derive(Clone)]
pub struct Matrix {
    row: [Vector4; 4],
}

impl Matrix {
    pub fn new(r0: Vector4, r1: Vector4, r2: Vector4, r3: Vector4) -> Matrix {
        Matrix {
            row: [r0, r1, r2, r3],
        }
    }

    pub fn identity() -> Matrix {
        let r0 = Vector4::new(1.0, 0.0, 0.0, 0.0);
        let r1 = Vector4::new(0.0, 1.0, 0.0, 0.0);
        let r2 = Vector4::new(0.0, 0.0, 1.0, 0.0);
        let r3 = Vector4::new(0.0, 0.0, 0.0, 1.0);
        return Matrix {
            row: [r0, r1, r2, r3],
        };
    }

    pub fn zero() -> Matrix {
        Matrix {
            row: [Vector4::zero(); 4],
        }
    }

    pub fn column(&self, idx: usize) -> Vector4 {
        Vector4::new(
            self.row[0][idx],
            self.row[1][idx],
            self.row[2][idx],
            self.row[3][idx],
        )
    }

    pub fn transpose(&self) -> Matrix {
        Matrix::new(
            self.column(0),
            self.column(1),
            self.column(2),
            self.column(3),
        )
    }

    pub fn invert(&self) -> Matrix {
        let mut result = Matrix::zero();
        let m = self;

        // Taken and modified from http://stackoverflow.com/questions/1148309/inverting-a-4x4-matrix
        result[0][0] =
            m[1][1] * m[2][2] * m[3][3] - m[1][1] * m[2][3] * m[3][2] - m[2][1] * m[1][2] * m[3][3]
                + m[2][1] * m[1][3] * m[3][2]
                + m[3][1] * m[1][2] * m[2][3]
                - m[3][1] * m[1][3] * m[2][2];
        result[1][0] = -m[1][0] * m[2][2] * m[3][3]
            + m[1][0] * m[2][3] * m[3][2]
            + m[2][0] * m[1][2] * m[3][3]
            - m[2][0] * m[1][3] * m[3][2]
            - m[3][0] * m[1][2] * m[2][3]
            + m[3][0] * m[1][3] * m[2][2];
        result[2][0] =
            m[1][0] * m[2][1] * m[3][3] - m[1][0] * m[2][3] * m[3][1] - m[2][0] * m[1][1] * m[3][3]
                + m[2][0] * m[1][3] * m[3][1]
                + m[3][0] * m[1][1] * m[2][3]
                - m[3][0] * m[1][3] * m[2][1];
        result[3][0] = -m[1][0] * m[2][1] * m[3][2]
            + m[1][0] * m[2][2] * m[3][1]
            + m[2][0] * m[1][1] * m[3][2]
            - m[2][0] * m[1][2] * m[3][1]
            - m[3][0] * m[1][1] * m[2][2]
            + m[3][0] * m[1][2] * m[2][1];

        let determinant = m[0][0] * result[0][0]
            + m[0][1] * result[1][0]
            + m[0][2] * result[2][0]
            + m[0][3] * result[3][0];
        if determinant == 0.0 {
            return Matrix::zero();
        }

        result[0][1] = -m[0][1] * m[2][2] * m[3][3]
            + m[0][1] * m[2][3] * m[3][2]
            + m[2][1] * m[0][2] * m[3][3]
            - m[2][1] * m[0][3] * m[3][2]
            - m[3][1] * m[0][2] * m[2][3]
            + m[3][1] * m[0][3] * m[2][2];
        result[1][1] =
            m[0][0] * m[2][2] * m[3][3] - m[0][0] * m[2][3] * m[3][2] - m[2][0] * m[0][2] * m[3][3]
                + m[2][0] * m[0][3] * m[3][2]
                + m[3][0] * m[0][2] * m[2][3]
                - m[3][0] * m[0][3] * m[2][2];
        result[2][1] = -m[0][0] * m[2][1] * m[3][3]
            + m[0][0] * m[2][3] * m[3][1]
            + m[2][0] * m[0][1] * m[3][3]
            - m[2][0] * m[0][3] * m[3][1]
            - m[3][0] * m[0][1] * m[2][3]
            + m[3][0] * m[0][3] * m[2][1];
        result[3][1] =
            m[0][0] * m[2][1] * m[3][2] - m[0][0] * m[2][2] * m[3][1] - m[2][0] * m[0][1] * m[3][2]
                + m[2][0] * m[0][2] * m[3][1]
                + m[3][0] * m[0][1] * m[2][2]
                - m[3][0] * m[0][2] * m[2][1];
        result[0][2] =
            m[0][1] * m[1][2] * m[3][3] - m[0][1] * m[1][3] * m[3][2] - m[1][1] * m[0][2] * m[3][3]
                + m[1][1] * m[0][3] * m[3][2]
                + m[3][1] * m[0][2] * m[1][3]
                - m[3][1] * m[0][3] * m[1][2];
        result[1][2] = -m[0][0] * m[1][2] * m[3][3]
            + m[0][0] * m[1][3] * m[3][2]
            + m[1][0] * m[0][2] * m[3][3]
            - m[1][0] * m[0][3] * m[3][2]
            - m[3][0] * m[0][2] * m[1][3]
            + m[3][0] * m[0][3] * m[1][2];
        result[2][2] =
            m[0][0] * m[1][1] * m[3][3] - m[0][0] * m[1][3] * m[3][1] - m[1][0] * m[0][1] * m[3][3]
                + m[1][0] * m[0][3] * m[3][1]
                + m[3][0] * m[0][1] * m[1][3]
                - m[3][0] * m[0][3] * m[1][1];
        result[3][2] = -m[0][0] * m[1][1] * m[3][2]
            + m[0][0] * m[1][2] * m[3][1]
            + m[1][0] * m[0][1] * m[3][2]
            - m[1][0] * m[0][2] * m[3][1]
            - m[3][0] * m[0][1] * m[1][2]
            + m[3][0] * m[0][2] * m[1][1];
        result[0][3] = -m[0][1] * m[1][2] * m[2][3]
            + m[0][1] * m[1][3] * m[2][2]
            + m[1][1] * m[0][2] * m[2][3]
            - m[1][1] * m[0][3] * m[2][2]
            - m[2][1] * m[0][2] * m[1][3]
            + m[2][1] * m[0][3] * m[1][2];
        result[1][3] =
            m[0][0] * m[1][2] * m[2][3] - m[0][0] * m[1][3] * m[2][2] - m[1][0] * m[0][2] * m[2][3]
                + m[1][0] * m[0][3] * m[2][2]
                + m[2][0] * m[0][2] * m[1][3]
                - m[2][0] * m[0][3] * m[1][2];
        result[2][3] = -m[0][0] * m[1][1] * m[2][3]
            + m[0][0] * m[1][3] * m[2][1]
            + m[1][0] * m[0][1] * m[2][3]
            - m[1][0] * m[0][3] * m[2][1]
            - m[2][0] * m[0][1] * m[1][3]
            + m[2][0] * m[0][3] * m[1][1];
        result[3][3] =
            m[0][0] * m[1][1] * m[2][2] - m[0][0] * m[1][2] * m[2][1] - m[1][0] * m[0][1] * m[2][2]
                + m[1][0] * m[0][2] * m[2][1]
                + m[2][0] * m[0][1] * m[1][2]
                - m[2][0] * m[0][2] * m[1][1];

        result = result / determinant;
        return result;
    }

    pub fn determinant(&self) -> f32 {
        let mut result = Matrix::zero();
        let m = self;

        result[0][0] =
            m[1][1] * m[2][2] * m[3][3] - m[1][1] * m[2][3] * m[3][2] - m[2][1] * m[1][2] * m[3][3]
                + m[2][1] * m[1][3] * m[3][2]
                + m[3][1] * m[1][2] * m[2][3]
                - m[3][1] * m[1][3] * m[2][2];
        result[1][0] = -m[1][0] * m[2][2] * m[3][3]
            + m[1][0] * m[2][3] * m[3][2]
            + m[2][0] * m[1][2] * m[3][3]
            - m[2][0] * m[1][3] * m[3][2]
            - m[3][0] * m[1][2] * m[2][3]
            + m[3][0] * m[1][3] * m[2][2];
        result[2][0] =
            m[1][0] * m[2][1] * m[3][3] - m[1][0] * m[2][3] * m[3][1] - m[2][0] * m[1][1] * m[3][3]
                + m[2][0] * m[1][3] * m[3][1]
                + m[3][0] * m[1][1] * m[2][3]
                - m[3][0] * m[1][3] * m[2][1];
        result[3][0] = -m[1][0] * m[2][1] * m[3][2]
            + m[1][0] * m[2][2] * m[3][1]
            + m[2][0] * m[1][1] * m[3][2]
            - m[2][0] * m[1][2] * m[3][1]
            - m[3][0] * m[1][1] * m[2][2]
            + m[3][0] * m[1][2] * m[2][1];

        return m[0][0] * result[0][0]
            + m[0][1] * result[1][0]
            + m[0][2] * result[2][0]
            + m[0][3] * result[3][0];
    }
}

impl ops::Index<usize> for Matrix {
    type Output = Vector4;
    fn index(&self, idx: usize) -> &Vector4 {
        &self.row[idx]
    }
}

impl ops::IndexMut<usize> for Matrix {
    fn index_mut(&mut self, idx: usize) -> &mut Vector4 {
        &mut self.row[idx]
    }
}

impl ops::Mul<f32> for Matrix {
    type Output = Matrix;
    fn mul(self, scalar: f32) -> Matrix {
        Matrix {
            row: [
                self.row[0] * scalar,
                self.row[1] * scalar,
                self.row[2] * scalar,
                self.row[3] * scalar,
            ],
        }
    }
}

impl ops::Mul<Matrix> for f32 {
    type Output = Matrix;
    fn mul(self, m: Matrix) -> Matrix {
        return m * self;
    }
}

impl ops::Mul<Vector4> for Matrix {
    type Output = Vector4;
    fn mul(self, f4: Vector4) -> Vector4 {
        let mut product = Vector4::zero();
        for idx in 0..4 {
            product[idx] = dot(self.row[idx], f4);
        }
        return product;
    }
}

impl ops::Mul<Vector3> for Matrix {
    type Output = Vector3;
    fn mul(self, v: Vector3) -> Vector3 {
        return Vector3::from(self * Vector4::from(v));
    }
}

impl ops::Mul<Point> for Matrix {
    type Output = Point;
    fn mul(self, p: Point) -> Point {
        return Point::from(self * Vector4::from(p));
    }
}

impl ops::Div<f32> for Matrix {
    type Output = Matrix;
    fn div(self, divisor: f32) -> Matrix {
        Matrix {
            row: [
                self.row[0] / divisor,
                self.row[1] / divisor,
                self.row[2] / divisor,
                self.row[3] / divisor,
            ],
        }
    }
}

impl ops::Mul<Matrix> for Matrix {
    type Output = Matrix;
    fn mul(self, rhs: Matrix) -> Matrix {
        let mut result = Matrix::zero();
        for idx in 0..4 {
            result[idx] = Vector4::new(
                dot(self.row[idx], rhs.column(0)),
                dot(self.row[idx], rhs.column(1)),
                dot(self.row[idx], rhs.column(2)),
                dot(self.row[idx], rhs.column(3)),
            );
        }

        return result;
    }
}
