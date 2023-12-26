use crate::pbrt::*;

// TODO: remove Copy from Transform
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

    pub fn from_matrix(matrix: SquareMatrix<4>) -> Self {
        return Transform {
            matrix,
            inverted_matrix: matrix.inverse(),
        };
    }

    pub fn from_array(array: [[f64; 4]; 4]) -> Self {
        let matrix = SquareMatrix::<4>::new(array);
        return Transform::from_matrix(matrix);
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

    pub fn transpose(&self) -> Transform {
        return Transform {
            matrix: self.matrix.transpose(),
            inverted_matrix: self.inverted_matrix.transpose(),
        };
    }

    pub fn translate(x: f64, y: f64, z: f64) -> Self {
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

    pub fn rotate(angle: f64, x: f64, y: f64, z: f64) -> Self {
        let sin_theta = degree_to_radian(angle).sin();
        let cos_theta = degree_to_radian(angle).cos();
        let axis = Vector3f::new(x, y, z).normalize();

        let mut m = SquareMatrix::<4>::identity();
        // Compute rotation of first basis vector
        m[0][0] = axis.x * axis.x + (1.0 - axis.x * axis.x) * cos_theta;
        m[0][1] = axis.x * axis.y * (1.0 - cos_theta) - axis.z * sin_theta;
        m[0][2] = axis.x * axis.z * (1.0 - cos_theta) + axis.y * sin_theta;
        m[0][3] = 0.0;

        // Compute rotations of second and third basis vectors
        m[1][0] = axis.x * axis.y * (1.0 - cos_theta) + axis.z * sin_theta;
        m[1][1] = axis.y * axis.y + (1.0 - axis.y * axis.y) * cos_theta;
        m[1][2] = axis.y * axis.z * (1.0 - cos_theta) - axis.x * sin_theta;
        m[1][3] = 0.0;

        m[2][0] = axis.x * axis.z * (1.0 - cos_theta) - axis.y * sin_theta;
        m[2][1] = axis.y * axis.z * (1.0 - cos_theta) + axis.x * sin_theta;
        m[2][2] = axis.z * axis.z + (1.0 - axis.z * axis.z) * cos_theta;
        m[2][3] = 0.0;

        return Transform {
            matrix: m.clone(),
            inverted_matrix: m.transpose(),
        };
    }

    pub fn rotate_from_to(from: Vector3f, to: Vector3f) -> Transform {
        // Compute intermediate vector for vector reflection
        let threshold = 0.72;
        let refl = if from.x.abs() < threshold && to.x.abs() < threshold {
            Vector3f::new(1.0, 0.0, 0.0)
        } else if from.y.abs() < threshold && to.y.abs() < threshold {
            Vector3f::new(0.0, 1.0, 0.0)
        } else {
            Vector3f::new(0.0, 0.0, 1.0)
        };

        let u = refl - from;
        let v = refl - to;
        let mut r = SquareMatrix::<4>::identity();

        for i in 0..3 {
            for j in 0..3 {
                // Initialize matrix element _r[i][j]_

                r[i][j] = if i == j { 1.0 } else { 0.0 }
                    - 2.0 / u.dot(u) * u[i] * u[j]
                    - 2.0 / v.dot(v) * v[i] * v[j]
                    + 4.0 * u.dot(v) / (u.dot(u) * v.dot(v)) * v[i] * u[j];
            }
        }

        return Transform {
            matrix: r,
            inverted_matrix: r.transpose(),
        };
    }

    pub fn scale(x: f64, y: f64, z: f64) -> Transform {
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

    pub fn perspective(fov: f64, z_near: f64, z_far: f64) -> Transform {
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

        let inv_tan_ang = 1.0 / (degree_to_radian(fov) / 2.0).tan();
        return Transform::scale(inv_tan_ang, inv_tan_ang, 1.0) * Transform::from_matrix(persp);
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
        let x = p.x.midpoint();
        let y = p.y.midpoint();
        let z = p.z.midpoint();

        let m = self.matrix;
        // Compute transformed coordinates from point _(x, y, z)_
        let xp = (m[0][0] * x + m[0][1] * y) + (m[0][2] * z + m[0][3]);
        let yp = (m[1][0] * x + m[1][1] * y) + (m[1][2] * z + m[1][3]);
        let zp = (m[2][0] * x + m[2][1] * y) + (m[2][2] * z + m[2][3]);
        let wp = (m[3][0] * x + m[3][1] * y) + (m[3][2] * z + m[3][3]);

        // Compute absolute error for transformed point, _pError_
        let p_error = if p.is_exact() {
            let _x = gamma(3)
                * ((m[0][0] * x).abs() + (m[0][1] * y).abs() + (m[0][2] * z).abs() + m[0][3].abs());
            let _y = gamma(3)
                * ((m[1][0] * x).abs() + (m[1][1] * y).abs() + (m[1][2] * z).abs() + m[1][3].abs());

            let _z = gamma(3)
                * ((m[2][0] * x).abs() + (m[2][1] * y).abs() + (m[2][2] * z).abs() + m[2][3].abs());
            Vector3f::new(_x, _y, _z)
        } else {
            // Compute error for transformed approximate _p_
            let p_in_error = p.error();
            let _x = (gamma(3) + 1.0)
                * (m[0][0].abs() * p_in_error.x
                    + m[0][1].abs() * p_in_error.y
                    + m[0][2].abs() * p_in_error.z)
                + gamma(3)
                    * ((m[0][0] * x).abs()
                        + (m[0][1] * y).abs()
                        + (m[0][2] * z).abs()
                        + m[0][3].abs());
            let _y = (gamma(3) + 1.0)
                * (m[1][0].abs() * p_in_error.x
                    + m[1][1].abs() * p_in_error.y
                    + m[1][2].abs() * p_in_error.z)
                + gamma(3)
                    * ((m[1][0] * x).abs()
                        + (m[1][1] * y).abs()
                        + (m[1][2] * z).abs()
                        + m[1][3].abs());
            let _z = (gamma(3) + 1.0)
                * (m[2][0].abs() * p_in_error.x
                    + m[2][1].abs() * p_in_error.y
                    + m[2][2].abs() * p_in_error.z)
                + gamma(3)
                    * ((m[2][0] * x).abs()
                        + (m[2][1] * y).abs()
                        + (m[2][2] * z).abs()
                        + m[2][3].abs());

            Vector3f::new(_x, _y, _z)
        };

        return if wp == 1.0 {
            Point3fi::from_value_and_error(Point3f::new(xp, yp, zp), p_error)
        } else {
            Point3fi::from_value_and_error(Point3f::new(xp, yp, zp), p_error) / wp
        };
    }

    pub fn on_vector3f(&self, v: Vector3f) -> Vector3f {
        let m = self.matrix;
        return Vector3f::new(
            m[0][0] * v.x + m[0][1] * v.y + m[0][2] * v.z,
            m[1][0] * v.x + m[1][1] * v.y + m[1][2] * v.z,
            m[2][0] * v.x + m[2][1] * v.y + m[2][2] * v.z,
        );
    }

    pub fn inverse_on_vector3f(&self, v: Vector3f) -> Vector3f {
        let m_inv = self.inverted_matrix;
        return Vector3f::new(
            m_inv[0][0] * v.x + m_inv[0][1] * v.y + m_inv[0][2] * v.z,
            m_inv[1][0] * v.x + m_inv[1][1] * v.y + m_inv[1][2] * v.z,
            m_inv[2][0] * v.x + m_inv[2][1] * v.y + m_inv[2][2] * v.z,
        );
    }

    pub fn on_vector3fi(&self, v: Vector3fi) -> Vector3fi {
        let x = v.x.midpoint();
        let y = v.y.midpoint();
        let z = v.z.midpoint();

        let m = self.matrix;

        let v_out_error = if v.is_exact() {
            let _x = gamma(3) * ((m[0][0] * x).abs() + (m[0][1] * y).abs() + (m[0][2] * z).abs());
            let _y = gamma(3) * ((m[1][0] * x).abs() + (m[1][1] * y).abs() + (m[1][2] * z).abs());
            let _z = gamma(3) * ((m[2][0] * x).abs() + (m[2][1] * y).abs() + (m[2][2] * z).abs());
            Vector3f::new(_x, _y, _z)
        } else {
            let v_in_error = v.error();
            let _x = (gamma(3) + 1.0)
                * (m[0][0].abs() * v_in_error.x
                    + m[0][1].abs() * v_in_error.y
                    + m[0][2].abs() * v_in_error.z)
                + gamma(3) * ((m[0][0] * x).abs() + (m[0][1] * y).abs() + (m[0][2] * z).abs());
            let _y = (gamma(3) + 1.0)
                * (m[1][0].abs() * v_in_error.x
                    + m[1][1].abs() * v_in_error.y
                    + m[1][2].abs() * v_in_error.z)
                + gamma(3) * ((m[1][0] * x).abs() + (m[1][1] * y).abs() + (m[1][2] * z).abs());
            let _z = (gamma(3) + 1.0)
                * (m[2][0].abs() * v_in_error.x
                    + m[2][1].abs() * v_in_error.y
                    + m[2][2].abs() * v_in_error.z)
                + gamma(3) * ((m[2][0] * x).abs() + (m[2][1] * y).abs() + (m[2][2] * z).abs());
            Vector3f::new(_x, _y, _z)
        };

        let xp = m[0][0] * x + m[0][1] * y + m[0][2] * z;
        let yp = m[1][0] * x + m[1][1] * y + m[1][2] * z;
        let zp = m[2][0] * x + m[2][1] * y + m[2][2] * z;

        return Vector3fi::new_with_error(Vector3f::new(xp, yp, zp), v_out_error);
    }

    pub fn on_normal3f(&self, n: Normal3f) -> Normal3f {
        let x = n.x;
        let y = n.y;
        let z = n.z;

        return Normal3f {
            x: self.inverted_matrix[0][0] * x
                + self.inverted_matrix[1][0] * y
                + self.inverted_matrix[2][0] * z,
            y: self.inverted_matrix[0][1] * x
                + self.inverted_matrix[1][1] * y
                + self.inverted_matrix[2][1] * z,
            z: self.inverted_matrix[0][2] * x
                + self.inverted_matrix[1][2] * y
                + self.inverted_matrix[2][2] * z,
        };
    }

    pub fn on_bounds(&self, bounds: Bounds3f) -> Bounds3f {
        // a smarter way to transform bounds:
        // takes roughly 2 transforms instead of 8
        // https://stackoverflow.com/a/58630206

        let mut transformed_bounds = Bounds3f::empty();
        for idx in 0..3 {
            transformed_bounds.p_min[idx] = self.matrix[idx][3];
        }
        transformed_bounds.p_max = transformed_bounds.p_min;

        for i in 0..3 {
            for k in 0..3 {
                let a = self.matrix[i][k] * bounds.p_min[k];
                let b = self.matrix[i][k] * bounds.p_max[k];

                let (min_val, max_val) = if a < b { (a, b) } else { (b, a) };
                transformed_bounds.p_min[i] += min_val;
                transformed_bounds.p_max[i] += max_val;
            }
        }

        return transformed_bounds;
    }

    pub fn on_ray(&self, r: &Ray) -> (Ray, f64) {
        let o = self.on_point3fi(Point3fi::from(r.o));
        let d = self.on_vector3f(r.d);

        let length_squared = d.length_squared();
        if length_squared <= 0.0 {
            panic!("illegal Ray");
        }

        let dt = d.abs().dot(o.error()) / length_squared;
        let offset_o = o + Vector3fi::from(d * dt);

        return (Ray::new(offset_o.into(), d), dt);
    }

    pub fn on_differential_ray(&self, r: &DifferentialRay) -> (DifferentialRay, f64) {
        let (transformed_ray, dt) = self.on_ray(&r.ray);

        return (
            DifferentialRay {
                ray: transformed_ray,
                has_differentials: true,
                rx_origin: self.on_point3f(r.rx_origin),
                ry_origin: self.on_point3f(r.ry_origin),
                rx_direction: self.on_vector3f(r.rx_direction),
                ry_direction: self.on_vector3f(r.ry_direction),
            },
            dt,
        );
    }

    pub fn on_shading(&self, shading: &Shading) -> Shading {
        return Shading {
            n: self.on_normal3f(shading.n).normalize(),
            dpdu: self.on_vector3f(shading.dpdu),
            dpdv: self.on_vector3f(shading.dpdv),
            dndu: self.on_vector3f(shading.dndu),
            dndv: self.on_vector3f(shading.dndv),
        };
    }

    pub fn on_interaction(&self, interaction: Interaction) -> Interaction {
        return Interaction {
            pi: self.on_point3fi(interaction.pi),
            n: self.on_normal3f(interaction.n),
            wo: self.on_vector3f(interaction.wo).normalize(),
            uv: interaction.uv,
        };
    }

    pub fn on_surface_interaction(&self, si: SurfaceInteraction) -> SurfaceInteraction {
        let mut shading = self.on_shading(&si.shading);
        shading.n = shading.n.face_forward(Vector3::from(si.interaction.n));

        return SurfaceInteraction {
            interaction: self.on_interaction(si.interaction),

            dpdx: self.on_vector3f(si.dpdx),
            dpdy: self.on_vector3f(si.dpdy),
            dpdu: self.on_vector3f(si.dpdu),
            dpdv: self.on_vector3f(si.dpdv),
            dndu: self.on_normal3f(si.dndu),
            dndv: self.on_normal3f(si.dndv),
            dudx: si.dudx,
            dvdx: si.dvdx,
            dudy: si.dudy,
            dvdy: si.dvdy,
            shading,
            material: si.material,
            area_light: si.area_light,
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

impl Display for Transform {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "matrix:\n{}inverted matrix:\n{}",
            self.matrix, self.inverted_matrix
        )
    }
}
