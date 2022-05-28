use std::ops;

use crate::fundamental::point::Point;
use crate::fundamental::vector3::Vector3;
use crate::fundamental::vector4::*;

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
            row: [Vector4::zero(); 4]
        }
    }

    pub fn column(&self, idx: usize) -> Vector4 {
        Vector4::new(self.row[0][idx],
                     self.row[1][idx],
                     self.row[2][idx],
                     self.row[3][idx])
    }

    pub fn transpose(&self) -> Matrix {
        Matrix::new(self.column(0), self.column(1), self.column(2), self.column(3))
    }

    pub fn invert(&self) -> Matrix {
        let mut result = Matrix::zero();
        let m = self;

        // Taken and modified from http://stackoverflow.com/questions/1148309/inverting-a-4x4-matrix
        result[0][0] = m[1][1] * m[2][2] * m[3][3] - m[1][1] * m[2][3] * m[3][2] - m[2][1] * m[1][2] * m[3][3] +
            m[2][1] * m[1][3] * m[3][2] + m[3][1] * m[1][2] * m[2][3] - m[3][1] * m[1][3] * m[2][2];
        result[1][0] = -m[1][0] * m[2][2] * m[3][3] + m[1][0] * m[2][3] * m[3][2] + m[2][0] * m[1][2] * m[3][3] -
            m[2][0] * m[1][3] * m[3][2] - m[3][0] * m[1][2] * m[2][3] + m[3][0] * m[1][3] * m[2][2];
        result[2][0] = m[1][0] * m[2][1] * m[3][3] - m[1][0] * m[2][3] * m[3][1] - m[2][0] * m[1][1] * m[3][3] +
            m[2][0] * m[1][3] * m[3][1] + m[3][0] * m[1][1] * m[2][3] - m[3][0] * m[1][3] * m[2][1];
        result[3][0] = -m[1][0] * m[2][1] * m[3][2] + m[1][0] * m[2][2] * m[3][1] + m[2][0] * m[1][1] * m[3][2] -
            m[2][0] * m[1][2] * m[3][1] - m[3][0] * m[1][1] * m[2][2] + m[3][0] * m[1][2] * m[2][1];

        let determinant = m[0][0] * result[0][0] + m[0][1] * result[1][0] + m[0][2] * result[2][0] + m[0][3] * result[3][0];
        if determinant == 0.0 {
            return Matrix::zero();
        }

        result[0][1] = -m[0][1] * m[2][2] * m[3][3] + m[0][1] * m[2][3] * m[3][2] + m[2][1] * m[0][2] * m[3][3] -
            m[2][1] * m[0][3] * m[3][2] - m[3][1] * m[0][2] * m[2][3] + m[3][1] * m[0][3] * m[2][2];
        result[1][1] = m[0][0] * m[2][2] * m[3][3] - m[0][0] * m[2][3] * m[3][2] - m[2][0] * m[0][2] * m[3][3] +
            m[2][0] * m[0][3] * m[3][2] + m[3][0] * m[0][2] * m[2][3] - m[3][0] * m[0][3] * m[2][2];
        result[2][1] = -m[0][0] * m[2][1] * m[3][3] + m[0][0] * m[2][3] * m[3][1] + m[2][0] * m[0][1] * m[3][3] -
            m[2][0] * m[0][3] * m[3][1] - m[3][0] * m[0][1] * m[2][3] + m[3][0] * m[0][3] * m[2][1];
        result[3][1] = m[0][0] * m[2][1] * m[3][2] - m[0][0] * m[2][2] * m[3][1] - m[2][0] * m[0][1] * m[3][2] +
            m[2][0] * m[0][2] * m[3][1] + m[3][0] * m[0][1] * m[2][2] - m[3][0] * m[0][2] * m[2][1];
        result[0][2] = m[0][1] * m[1][2] * m[3][3] - m[0][1] * m[1][3] * m[3][2] - m[1][1] * m[0][2] * m[3][3] +
            m[1][1] * m[0][3] * m[3][2] + m[3][1] * m[0][2] * m[1][3] - m[3][1] * m[0][3] * m[1][2];
        result[1][2] = -m[0][0] * m[1][2] * m[3][3] + m[0][0] * m[1][3] * m[3][2] + m[1][0] * m[0][2] * m[3][3] -
            m[1][0] * m[0][3] * m[3][2] - m[3][0] * m[0][2] * m[1][3] + m[3][0] * m[0][3] * m[1][2];
        result[2][2] = m[0][0] * m[1][1] * m[3][3] - m[0][0] * m[1][3] * m[3][1] - m[1][0] * m[0][1] * m[3][3] +
            m[1][0] * m[0][3] * m[3][1] + m[3][0] * m[0][1] * m[1][3] - m[3][0] * m[0][3] * m[1][1];
        result[3][2] = -m[0][0] * m[1][1] * m[3][2] + m[0][0] * m[1][2] * m[3][1] + m[1][0] * m[0][1] * m[3][2] -
            m[1][0] * m[0][2] * m[3][1] - m[3][0] * m[0][1] * m[1][2] + m[3][0] * m[0][2] * m[1][1];
        result[0][3] = -m[0][1] * m[1][2] * m[2][3] + m[0][1] * m[1][3] * m[2][2] + m[1][1] * m[0][2] * m[2][3] -
            m[1][1] * m[0][3] * m[2][2] - m[2][1] * m[0][2] * m[1][3] + m[2][1] * m[0][3] * m[1][2];
        result[1][3] = m[0][0] * m[1][2] * m[2][3] - m[0][0] * m[1][3] * m[2][2] - m[1][0] * m[0][2] * m[2][3] +
            m[1][0] * m[0][3] * m[2][2] + m[2][0] * m[0][2] * m[1][3] - m[2][0] * m[0][3] * m[1][2];
        result[2][3] = -m[0][0] * m[1][1] * m[2][3] + m[0][0] * m[1][3] * m[2][1] + m[1][0] * m[0][1] * m[2][3] -
            m[1][0] * m[0][3] * m[2][1] - m[2][0] * m[0][1] * m[1][3] + m[2][0] * m[0][3] * m[1][1];
        result[3][3] = m[0][0] * m[1][1] * m[2][2] - m[0][0] * m[1][2] * m[2][1] - m[1][0] * m[0][1] * m[2][2] +
            m[1][0] * m[0][2] * m[2][1] + m[2][0] * m[0][1] * m[1][2] - m[2][0] * m[0][2] * m[1][1];

        result = result / determinant;
        return result;
    }

    pub fn determinant(&self) -> f32 {
        let mut result = Matrix::zero();
        let m = self;

        result[0][0] = m[1][1] * m[2][2] * m[3][3] - m[1][1] * m[2][3] * m[3][2] - m[2][1] * m[1][2] * m[3][3] +
            m[2][1] * m[1][3] * m[3][2] + m[3][1] * m[1][2] * m[2][3] - m[3][1] * m[1][3] * m[2][2];
        result[1][0] = -m[1][0] * m[2][2] * m[3][3] + m[1][0] * m[2][3] * m[3][2] + m[2][0] * m[1][2] * m[3][3] -
            m[2][0] * m[1][3] * m[3][2] - m[3][0] * m[1][2] * m[2][3] + m[3][0] * m[1][3] * m[2][2];
        result[2][0] = m[1][0] * m[2][1] * m[3][3] - m[1][0] * m[2][3] * m[3][1] - m[2][0] * m[1][1] * m[3][3] +
            m[2][0] * m[1][3] * m[3][1] + m[3][0] * m[1][1] * m[2][3] - m[3][0] * m[1][3] * m[2][1];
        result[3][0] = -m[1][0] * m[2][1] * m[3][2] + m[1][0] * m[2][2] * m[3][1] + m[2][0] * m[1][1] * m[3][2] -
            m[2][0] * m[1][2] * m[3][1] - m[3][0] * m[1][1] * m[2][2] + m[3][0] * m[1][2] * m[2][1];

        return m[0][0] * result[0][0] + m[0][1] * result[1][0] + m[0][2] * result[2][0] + m[0][3] * result[3][0];
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
            result[idx] = Vector4::new(dot(self.row[idx], rhs.column(0)),
                                       dot(self.row[idx], rhs.column(1)),
                                       dot(self.row[idx], rhs.column(2)),
                                       dot(self.row[idx], rhs.column(3)));
        }

        return result;
    }
}
