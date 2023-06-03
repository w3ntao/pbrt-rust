use crate::pbrt::*;

#[derive(Copy, Clone)]
pub struct Transform {
    matrix: SquareMatrix<4>,
    inverted_matrix: SquareMatrix<4>,
}

impl Transform {
    pub fn identity() -> Self {
        let identity_matrix = SquareMatrix::<4>::identity();
        return Transform {
            matrix: identity_matrix.clone(),
            inverted_matrix: identity_matrix,
        };
    }

    pub fn nan() -> Self {
        let nan_matrix = SquareMatrix::<4>::nan();

        return Transform {
            matrix: nan_matrix,
            inverted_matrix: nan_matrix,
        };
    }

    pub fn new(_matrix: SquareMatrix<4>) -> Self {
        return Transform {
            matrix: _matrix,
            inverted_matrix: SquareMatrix::<4>::nan(),
        };
    }

    pub fn new_with_inverse(_matrix: SquareMatrix<4>, _inv_matrix: SquareMatrix<4>) -> Self {
        return Transform {
            matrix: _matrix,
            inverted_matrix: _inv_matrix,
        };
    }

    pub fn inverse(&self) -> Transform {
        return Transform {
            matrix: self.inverted_matrix,
            inverted_matrix: self.matrix,
        };
    }

    pub fn translate(delta: Vector3f) -> Self {
        let mut _matrix = SquareMatrix::<4>::identity();
        let mut _inverted_matrix = _matrix.clone();

        for idx in 0..3 {
            _matrix[idx][3] += delta[idx];
            _inverted_matrix[idx][3] -= delta[idx];
        }

        return Transform {
            matrix: _matrix,
            inverted_matrix: _inverted_matrix,
        };
    }

    pub fn on_point(&self, p: Point3f) -> Point3f {
        let xp = self.matrix[0][0] * p.x
            + self.matrix[0][1] * p.y
            + self.matrix[0][2] * p.z
            + self.matrix[0][3];
        let yp = self.matrix[1][0] * p.x
            + self.matrix[1][1] * p.y
            + self.matrix[1][2] * p.z
            + self.matrix[1][3];
        let zp = self.matrix[2][0] * p.x
            + self.matrix[2][1] * p.y
            + self.matrix[2][2] * p.z
            + self.matrix[2][3];
        let wp = self.matrix[3][0] * p.x
            + self.matrix[3][1] * p.y
            + self.matrix[3][2] * p.z
            + self.matrix[3][3];

        return if wp == 1.0 {
            Point3f::new(xp, yp, zp)
        } else {
            Point3f::new(xp, yp, zp) / wp
        };
    }
}

impl Mul<Transform> for Transform {
    type Output = Transform;

    fn mul(self, rhs: Transform) -> Self::Output {
        return Transform {
            matrix: self.matrix * rhs.matrix,
            inverted_matrix: rhs.inverted_matrix * self.inverted_matrix,
        };
    }
}
