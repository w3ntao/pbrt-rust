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
