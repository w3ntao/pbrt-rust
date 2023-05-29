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

    pub fn new(_matrix: SquareMatrix<4>) -> Self {
        return Transform {
            matrix: _matrix,
            inverted_matrix: SquareMatrix::<4>::nan(),
        };
    }

    pub fn new_with_inv(_matrix: SquareMatrix<4>, _inv_matrix: SquareMatrix<4>) -> Self {
        return Transform {
            matrix: _matrix,
            inverted_matrix: _inv_matrix,
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
