use crate::core::pbrt::*;

#[derive(Clone, Copy)]
pub struct Transform {
    m: Matrix,
    inv_m: Matrix,
}

impl Transform {
    pub fn identity() -> Transform {
        let identity = Matrix::identity();
        return Transform {
            m: identity,
            inv_m: identity,
        };
    }

    pub fn inverse(&self) -> Transform {
        return Transform {
            m: self.inv_m,
            inv_m: self.m,
        };
    }

    pub fn determinant(&self) -> f32 {
        return self.m.determinant();
    }

    pub fn reset(&mut self) {
        *self = Transform::identity();
    }

    pub fn translate(&mut self, t: Vector3) {
        let mut matrix = Matrix::identity();
        for idx in 0..3 {
            matrix[idx][3] = t[idx];
        }

        self.m = matrix * self.m;
        self.inv_m = self.m.inverse();
    }

    pub fn scale_by_scalar(&mut self, scalar: f32) {
        let mut matrix = Matrix::identity();
        for idx in 0..3 {
            matrix[idx][idx] = scalar;
        }

        self.m = matrix * self.m;
        self.inv_m = self.m.inverse();
    }

    pub fn rotate(&mut self, axis: Vector3, angle: f32) {
        let cosine = f32::cos(angle);
        let sine = f32::sin(angle);

        let normalized_axis = axis.normalize();
        let x = normalized_axis.x;
        let y = normalized_axis.y;
        let z = normalized_axis.z;

        let mut rotate_matrix = Matrix::zero();
        rotate_matrix[0] = [
            x * x * (1.0 - cosine) + cosine,
            x * y * (1.0 - cosine) - z * sine,
            x * z * (1.0 - cosine) + y * sine,
            0.0,
        ];
        rotate_matrix[1] = [
            x * y * (1.0 - cosine) + z * sine,
            cosine + y * y * (1.0 - cosine),
            y * z * (1.0 - cosine) - x * sine,
            0.0,
        ];
        rotate_matrix[2] = [
            x * z * (1.0 - cosine) - y * sine,
            y * z * (1.0 - cosine) + x * sine,
            cosine + z * z * (1.0 - cosine),
            0.0,
        ];
        rotate_matrix[3] = [0.0, 0.0, 0.0, 1.0];

        self.m = rotate_matrix * self.m;
        self.inv_m = self.m.inverse();
    }

    pub fn on_point(self, p: Point) -> Point {
        // PBRT: Managing Rounding Error
        // https://www.pbr-book.org/3ed-2018/Shapes/Managing_Rounding_Error#x4-EffectofTransformations

        let x = p.x;
        let y = p.y;
        let z = p.z;

        // Compute transformed coordinates from point _pt_
        let xp = self.m[0][0] * x + self.m[0][1] * y + self.m[0][2] * z + self.m[0][3];
        let yp = self.m[1][0] * x + self.m[1][1] * y + self.m[1][2] * z + self.m[1][3];
        let zp = self.m[2][0] * x + self.m[2][1] * y + self.m[2][2] * z + self.m[2][3];
        let wp = self.m[3][0] * x + self.m[3][1] * y + self.m[3][2] * z + self.m[3][3];
        assert_ne!(wp, 0.0);

        if wp == 1.0 {
            return Point::new(xp, yp, zp);
        }
        return Point::new(xp, yp, zp) / wp;
    }

    pub fn on_point_get_error(self, p: Point, p_error: &mut Vector3) -> Point {
        // PBRT: Managing Rounding Error
        // https://www.pbr-book.org/3ed-2018/Shapes/Managing_Rounding_Error#x4-EffectofTransformations

        let x = p.x;
        let y = p.y;
        let z = p.z;

        // Compute absolute error for transformed point
        let xp = self.m[0][0] * x + self.m[0][1] * y + self.m[0][2] * z + self.m[0][3];
        let yp = self.m[1][0] * x + self.m[1][1] * y + self.m[1][2] * z + self.m[1][3];
        let zp = self.m[2][0] * x + self.m[2][1] * y + self.m[2][2] * z + self.m[2][3];
        let wp = self.m[3][0] * x + self.m[3][1] * y + self.m[3][2] * z + self.m[3][3];
        assert_ne!(wp, 0.0);

        // Compute absolute error for transformed point
        let x_abs_sum = (self.m[0][0] * x).abs()
            + (self.m[0][1] * y).abs()
            + (self.m[0][2] * z).abs()
            + self.m[0][3].abs();

        let y_abs_sum = (self.m[1][0] * x).abs()
            + (self.m[1][1] * y).abs()
            + (self.m[1][2] * z).abs()
            + self.m[1][3].abs();

        let z_abs_sum = (self.m[2][0] * x).abs()
            + (self.m[2][1] * y).abs()
            + (self.m[2][2] * z).abs()
            + self.m[2][3].abs();

        *p_error = gamma(3) * Vector3::new(x_abs_sum, y_abs_sum, z_abs_sum);

        if wp == 1.0 {
            return Point::new(xp, yp, zp);
        }
        return Point::new(xp, yp, zp) / wp;
    }

    pub fn on_point_with_error_get_error(
        self,
        pt: Point,
        pt_error: Vector3,
        abs_error: &mut Vector3,
    ) -> Point {
        let x = pt.x;
        let y = pt.y;
        let z = pt.z;
        let xp = self.m[0][0] * x + self.m[0][1] * y + self.m[0][2] * z + self.m[0][3];
        let yp = self.m[1][0] * x + self.m[1][1] * y + self.m[1][2] * z + self.m[1][3];
        let zp = self.m[2][0] * x + self.m[2][1] * y + self.m[2][2] * z + self.m[2][3];
        let wp = self.m[3][0] * x + self.m[3][1] * y + self.m[3][2] * z + self.m[3][3];

        abs_error.x = (gamma(3) + 1.0)
            * (self.m[0][0].abs() * pt_error.x
                + self.m[0][1].abs() * pt_error.y
                + self.m[0][2].abs() * pt_error.z)
            + gamma(3)
                * ((self.m[0][0] * x).abs()
                    + (self.m[0][1] * y).abs()
                    + (self.m[0][2] * z).abs()
                    + self.m[0][3].abs());

        abs_error.y = (gamma(3) + 1.0)
            * (self.m[1][0].abs() * pt_error.x
                + self.m[1][1].abs() * pt_error.y
                + self.m[1][2].abs() * pt_error.z)
            + gamma(3)
                * ((self.m[1][0] * x).abs()
                    + (self.m[1][1] * y).abs()
                    + (self.m[1][2] * z).abs()
                    + self.m[1][3].abs());

        abs_error.z = (gamma(3) + 1.0)
            * (self.m[2][0].abs() * pt_error.x
                + self.m[2][1].abs() * pt_error.y
                + self.m[2][2].abs() * pt_error.z)
            + gamma(3)
                * ((self.m[2][0] * x).abs()
                    + (self.m[2][1] * y).abs()
                    + (self.m[2][2] * z).abs()
                    + self.m[2][3].abs());

        assert_ne!(wp, 0.0);

        if wp == 1.0 {
            return Point::new(xp, yp, zp);
        }
        return Point::new(xp, yp, zp) / wp;
    }

    pub fn on_vector(self, v: Vector3) -> Vector3 {
        let x = v.x;
        let y = v.y;
        let z = v.z;

        return Vector3::new(
            self.m[0][0] * x + self.m[0][1] * y + self.m[0][2] * z,
            self.m[1][0] * x + self.m[1][1] * y + self.m[1][2] * z,
            self.m[2][0] * x + self.m[2][1] * y + self.m[2][2] * z,
        );
    }

    pub fn on_vector_get_error(self, v: Vector3, abs_error: &mut Vector3) -> Vector3 {
        let x = v.x;
        let y = v.y;
        let z = v.z;

        abs_error.x = gamma(3)
            * ((self.m[0][0] * v.x).abs()
                + (self.m[0][1] * v.y).abs()
                + (self.m[0][2] * v.z).abs());

        abs_error.y = gamma(3)
            * ((self.m[1][0] * v.x).abs()
                + (self.m[1][1] * v.y).abs()
                + (self.m[1][2] * v.z).abs());

        abs_error.z = gamma(3)
            * ((self.m[2][0] * v.x).abs()
                + (self.m[2][1] * v.y).abs()
                + (self.m[2][2] * v.z).abs());

        return Vector3::new(
            self.m[0][0] * x + self.m[0][1] * y + self.m[0][2] * z,
            self.m[1][0] * x + self.m[1][1] * y + self.m[1][2] * z,
            self.m[2][0] * x + self.m[2][1] * y + self.m[2][2] * z,
        );
    }

    pub fn on_normal(self, n: Normal) -> Normal {
        let x = n.x;
        let y = n.y;
        let z = n.z;

        let inverse_matrix = self.inv_m;
        return Normal::new(
            inverse_matrix[0][0] * x + inverse_matrix[1][0] * y + inverse_matrix[2][0] * z,
            inverse_matrix[0][1] * x + inverse_matrix[1][1] * y + inverse_matrix[2][1] * z,
            inverse_matrix[0][2] * x + inverse_matrix[1][2] * y + inverse_matrix[2][2] * z,
        )
        .normalize();
    }

    pub fn on_bounds(self, bounds: Bounds) -> Bounds {
        // a smarter way to transform bounds:
        // takes roughly 2 transforms instead of 8
        // https://stackoverflow.com/a/58630206

        let mut transformed_bounds = Bounds::empty();
        for idx in 0..3 {
            transformed_bounds.p_min[idx] = self.m[idx][3];
        }
        transformed_bounds.p_max = transformed_bounds.p_min;

        for i in 0..3 {
            for k in 0..3 {
                let a = self.m[i][k] * bounds.p_min[k];
                let b = self.m[i][k] * bounds.p_max[k];

                transformed_bounds.p_min[i] += if a < b { a } else { b };
                transformed_bounds.p_max[i] += if a < b { b } else { a };
            }
        }

        return transformed_bounds;
    }

    pub fn on_ray(self, ray: Ray) -> Ray {
        let mut o_error = Vector3::invalid();
        let o = self.on_point_get_error(ray.o, &mut o_error);
        let d = self.on_vector(ray.d);

        let length_squared = d.length_squared();
        let dt = d.abs().dot(o_error) / length_squared;

        return Ray::new(o + d * dt, d, ray.t_max - dt);
    }

    pub fn on_ray_get_error(self, ray: Ray, o_error: &mut Vector3, d_error: &mut Vector3) -> Ray {
        let o = self.on_point_get_error(ray.o, o_error);
        let d = self.on_vector_get_error(ray.d, d_error);

        let length_squared = d.length_squared();
        let dt = d.abs().dot(*o_error) / length_squared;

        return Ray::new(o + d * dt, d, ray.t_max);
    }

    pub fn on_surface_interaction(self, si: SurfaceInteraction) -> SurfaceInteraction {
        let mut ret = SurfaceInteraction::default();
        ret.p = self.on_point_with_error_get_error(si.p, si.p_error, &mut ret.p_error);
        ret.n = self.on_normal(si.n).normalize();
        ret.entering_material = si.entering_material;
        ret.material = si.material.clone();

        return ret;
    }
}
