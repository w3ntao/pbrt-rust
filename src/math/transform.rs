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

    pub fn is_identity(&self) -> bool {
        return self.matrix.is_identity() && self.inverted_matrix.is_identity();
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
            inverted_matrix: _matrix.inverse(),
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

    pub fn translate(x: Float, y: Float, z: Float) -> Self {
        let _matrix = SquareMatrix::new([
            [1.0, 0.0, 0.0, x],
            [0.0, 1.0, 0.0, y],
            [0.0, 0.0, 1.0, z],
            [0.0, 0.0, 0.0, 1.0],
        ]);

        let _inverted_matrix = SquareMatrix::new([
            [1.0, 0.0, 0.0, -x],
            [0.0, 1.0, 0.0, -y],
            [0.0, 0.0, 1.0, -z],
            [0.0, 0.0, 0.0, 1.0],
        ]);

        return Transform {
            matrix: _matrix,
            inverted_matrix: _inverted_matrix,
        };
    }

    pub fn scale(x: Float, y: Float, z: Float) -> Transform {
        let _matrix = SquareMatrix::<4>::new([
            [x, 0.0, 0.0, 0.0],
            [0.0, y, 0.0, 0.0],
            [0.0, 0.0, z, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ]);

        let _inverted_matrix = SquareMatrix::<4>::new([
            [1.0 / x, 0.0, 0.0, 0.0],
            [0.0, 1.0 / y, 0.0, 0.0],
            [0.0, 0.0, 1.0 / z, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ]);

        return Transform {
            matrix: _matrix,
            inverted_matrix: _inverted_matrix,
        };
    }

    pub fn perspective(fov: Float, z_near: Float, z_far: Float) -> Transform {
        let persp = SquareMatrix::<4>::new([
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [
                0.0,
                0.0,
                z_far / (z_far - z_near),
                -z_far * z_near / (z_far - z_near),
            ],
            [0.0, 0.0, 1.0, 0.0],
        ]);

        let invTanAng = 1.0 / (degree_to_radian(fov) / 2.0).tan();
        return Transform::scale(invTanAng, invTanAng, 1.0) * Transform::new(persp);
    }

    pub fn on_point3f(&self, p: Point3f) -> Point3f {
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

    pub fn on_point3fi(&self, p: Point3fi) -> Point3fi {
        return Point3fi::from(self.on_vector3fi(Vector3fi::from(p)));
    }

    pub fn on_vector3f(&self, v: Vector3f) -> Vector3f {
        let m = self.matrix;
        return Vector3f::new(
            m[0][0] * v.x + m[0][1] * v.y + m[0][2] * v.z,
            m[1][0] * v.x + m[1][1] * v.y + m[1][2] * v.z,
            m[2][0] * v.x + m[2][1] * v.y + m[2][2] * v.z,
        );
    }

    pub fn on_vector3fi(&self, v: Vector3fi) -> Vector3fi {
        let x = v.x.midpoint();
        let y = v.y.midpoint();
        let z = v.z.midpoint();

        let m = self.matrix;

        let (v_out_error_x, v_out_error_y, v_out_error_z) = if v.is_exact() {
            (
                gamma(3) * ((m[0][0] * x).abs() + (m[0][1] * y).abs() + (m[0][2] * z).abs()),
                gamma(3) * ((m[1][0] * x).abs() + (m[1][1] * y).abs() + (m[1][2] * z).abs()),
                gamma(3) * ((m[2][0] * x).abs() + (m[2][1] * y).abs() + (m[2][2] * z).abs()),
            )
        } else {
            let vInError = v.error();
            (
                (gamma(3) + 1.0)
                    * (m[0][0].abs() * vInError.x
                        + m[0][1].abs() * vInError.y
                        + m[0][2].abs() * vInError.z)
                    + gamma(3) * ((m[0][0] * x).abs() + (m[0][1] * y).abs() + (m[0][2] * z).abs()),
                (gamma(3) + 1.0)
                    * (m[1][0].abs() * vInError.x
                        + m[1][1].abs() * vInError.y
                        + m[1][2].abs() * vInError.z)
                    + gamma(3) * ((m[1][0] * x).abs() + (m[1][1] * y).abs() + (m[1][2] * z).abs()),
                (gamma(3) + 1.0)
                    * (m[2][0].abs() * vInError.x
                        + m[2][1].abs() * vInError.y
                        + m[2][2].abs() * vInError.z)
                    + gamma(3) * ((m[2][0] * x).abs() + (m[2][1] * y).abs() + (m[2][2] * z).abs()),
            )
        };

        let vOutError = Vector3f::new(v_out_error_x, v_out_error_y, v_out_error_z);

        let xp = m[0][0] * x + m[0][1] * y + m[0][2] * z;
        let yp = m[1][0] * x + m[1][1] * y + m[1][2] * z;
        let zp = m[2][0] * x + m[2][1] * y + m[2][2] * z;

        return Vector3fi::new_with_error(Vector3f::new(xp, yp, zp), vOutError);
    }

    pub fn on_ray(&self, r: Ray) -> (Ray, Float) {
        let o = self.on_point3fi(Point3fi::from(r.o));
        let d = self.on_vector3f(r.d);

        let length_squared = d.length_squared();
        if length_squared <= 0.0 {
            panic!("illegal Ray");
        }

        let dt = d.abs().dot(o.error()) / length_squared;
        let offset_o = o + Vector3fi::from(d * dt);

        return (Ray::new(Point3f::from(offset_o), d), dt);
    }

    pub fn on_ray_with_error(&self, r: Ray) -> (Ray, Float) {
        panic!("not implemented");
    }

    pub fn display(&self) {
        self.matrix.display();
        println!("inverted matrix:");
        self.inverted_matrix.display();
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
