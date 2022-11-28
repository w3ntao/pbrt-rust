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

    pub fn is_identity(&self) -> bool {
        return self.m.is_identity();
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
}

// https://docs.rs/fn_ops/latest/fn_ops/
impl FnOnce<(Point,)> for Transform {
    type Output = Point;
    extern "rust-call" fn call_once(self, _: (Point,)) -> Point {
        panic!("FnOnce<(Point,)> not implemented for Transform");
    }
}

impl FnMut<(Point,)> for Transform {
    extern "rust-call" fn call_mut(&mut self, _: (Point,)) -> Point {
        panic!("FnMut<(Point,)> not implemented for Transform");
    }
}

impl Fn<(Point,)> for Transform {
    extern "rust-call" fn call(&self, p: (Point,)) -> Point {
        let p = p.0;

        if self.is_identity() {
            return p;
        }

        // PBRT: Managing Rounding Error
        // https://www.pbr-book.org/3ed-2018/Shapes/Managing_Rounding_Error#x4-EffectofTransformations

        let x = p.x;
        let y = p.y;
        let z = p.z;

        // Compute transformed coordinates from point _pt_
        let xp = (self.m[0][0] * x + self.m[0][1] * y) + (self.m[0][2] * z + self.m[0][3]);
        let yp = (self.m[1][0] * x + self.m[1][1] * y) + (self.m[1][2] * z + self.m[1][3]);
        let zp = (self.m[2][0] * x + self.m[2][1] * y) + (self.m[2][2] * z + self.m[2][3]);
        let wp = (self.m[3][0] * x + self.m[3][1] * y) + (self.m[3][2] * z + self.m[3][3]);
        assert_ne!(wp, 0.0);

        if wp == 1.0 {
            return Point::new(xp, yp, zp);
        }
        return Point::new(xp, yp, zp) / wp;
    }
}

impl FnOnce<(Point, &mut Vector3)> for Transform {
    type Output = Point;
    extern "rust-call" fn call_once(self, _: (Point, &mut Vector3)) -> Point {
        panic!("FnOnce<(Point, &mut Vector3)> not implemented for Transform");
    }
}

impl FnMut<(Point, &mut Vector3)> for Transform {
    extern "rust-call" fn call_mut(&mut self, _: (Point, &mut Vector3)) -> Point {
        panic!("FnMut<(Point, &mut Vector3)> not implemented for Transform");
    }
}

impl Fn<(Point, &mut Vector3)> for Transform {
    extern "rust-call" fn call(&self, args: (Point, &mut Vector3)) -> Point {
        let (p_in, p_error) = args;

        if self.is_identity() {
            *p_error = Vector3::new(0.0, 0.0, 0.0);
            return p_in;
        }

        // PBRT: Managing Rounding Error
        // https://www.pbr-book.org/3ed-2018/Shapes/Managing_Rounding_Error#x4-EffectofTransformations

        let x = p_in.x;
        let y = p_in.y;
        let z = p_in.z;

        // Compute transformed coordinates from point _pt_
        let xp = (self.m[0][0] * x + self.m[0][1] * y) + (self.m[0][2] * z + self.m[0][3]);
        let yp = (self.m[1][0] * x + self.m[1][1] * y) + (self.m[1][2] * z + self.m[1][3]);
        let zp = (self.m[2][0] * x + self.m[2][1] * y) + (self.m[2][2] * z + self.m[2][3]);
        let wp = (self.m[3][0] * x + self.m[3][1] * y) + (self.m[3][2] * z + self.m[3][3]);
        assert_ne!(wp, 0.0);

        // Compute absolute error for transformed point
        let xAbsSum = (self.m[0][0].abs() * x)
            + (self.m[0][1].abs() * y)
            + (self.m[0][2].abs() * z)
            + (self.m[0][3]).abs();
        let yAbsSum = (self.m[1][0].abs() * x)
            + (self.m[1][1].abs() * y)
            + (self.m[1][2].abs() * z)
            + (self.m[1][3]).abs();
        let zAbsSum = (self.m[2][0].abs() * x)
            + (self.m[2][1].abs() * y)
            + (self.m[2][2].abs() * z)
            + (self.m[2][3]).abs();

        *p_error = gamma(3) * Vector3::new(xAbsSum, yAbsSum, zAbsSum);

        if wp == 1.0 {
            return Point::new(xp, yp, zp);
        }
        return Point::new(xp, yp, zp) / wp;
    }
}

impl FnOnce<(Point, Vector3, &mut Vector3)> for Transform {
    type Output = Point;
    extern "rust-call" fn call_once(self, _: (Point, Vector3, &mut Vector3)) -> Point {
        panic!("FnOnce<(Point, Vector3, &mut Vector3)> not implemented for Transform");
    }
}

impl FnMut<(Point, Vector3, &mut Vector3)> for Transform {
    extern "rust-call" fn call_mut(&mut self, _: (Point, Vector3, &mut Vector3)) -> Point {
        panic!("FnMut<(Point, Vector3, &mut Vector3)> not implemented for Transform");
    }
}

impl Fn<(Point, Vector3, &mut Vector3)> for Transform {
    extern "rust-call" fn call(&self, args: (Point, Vector3, &mut Vector3)) -> Point {
        let (pt, ptError, absError) = args;
        let x = pt.x;
        let y = pt.y;
        let z = pt.z;
        let xp = (self.m[0][0] * x + self.m[0][1] * y) + (self.m[0][2] * z + self.m[0][3]);
        let yp = (self.m[1][0] * x + self.m[1][1] * y) + (self.m[1][2] * z + self.m[1][3]);
        let zp = (self.m[2][0] * x + self.m[2][1] * y) + (self.m[2][2] * z + self.m[2][3]);
        let wp = (self.m[3][0] * x + self.m[3][1] * y) + (self.m[3][2] * z + self.m[3][3]);

        absError.x = (gamma(3) + 1.0)
            * (self.m[0][0].abs() * ptError.x
                + self.m[0][1].abs() * ptError.y
                + self.m[0][2].abs() * ptError.z)
            + gamma(3)
                * ((self.m[0][0] * x).abs()
                    + (self.m[0][1] * y).abs()
                    + (self.m[0][2] * z).abs()
                    + self.m[0][3].abs());

        absError.y = (gamma(3) + 1.0)
            * (self.m[1][0].abs() * ptError.x
                + self.m[1][1].abs() * ptError.y
                + self.m[1][2].abs() * ptError.z)
            + gamma(3)
                * ((self.m[1][0] * x).abs()
                    + (self.m[1][1] * y).abs()
                    + (self.m[1][2] * z).abs()
                    + (self.m[1][3]).abs());

        absError.z = (gamma(3) + 1.0)
            * (self.m[2][0].abs() * ptError.x
                + self.m[2][1].abs() * ptError.y
                + self.m[2][2].abs() * ptError.z)
            + gamma(3)
                * ((self.m[2][0] * x).abs()
                    + (self.m[2][1] * y).abs()
                    + (self.m[2][2] * z).abs()
                    + self.m[2][3].abs());

        if wp == 0.0 {
            panic!("error in transforming Point");
        }

        if wp == 1.0 {
            return Point::new(xp, yp, zp);
        }
        return Point::new(xp, yp, zp) / wp;
    }
}

impl FnOnce<(Vector3,)> for Transform {
    type Output = Vector3;
    extern "rust-call" fn call_once(self, _: (Vector3,)) -> Vector3 {
        panic!("FnOnce<(Vector3,)> not implemented for Transform");
    }
}

impl FnMut<(Vector3,)> for Transform {
    extern "rust-call" fn call_mut(&mut self, _: (Vector3,)) -> Vector3 {
        panic!("FnMut<(Vector3,)> not implemented for Transform");
    }
}

impl Fn<(Vector3,)> for Transform {
    extern "rust-call" fn call(&self, v: (Vector3,)) -> Vector3 {
        let v = v.0;
        if self.is_identity() {
            return v;
        }

        let x = v.x;
        let y = v.y;
        let z = v.z;

        let matrix = self.m;
        return Vector3::new(
            matrix[0][0] * x + matrix[0][1] * y + matrix[0][2] * z,
            matrix[1][0] * x + matrix[1][1] * y + matrix[1][2] * z,
            matrix[2][0] * x + matrix[2][1] * y + matrix[2][2] * z,
        );
    }
}

impl FnOnce<(Vector3, &mut Vector3)> for Transform {
    type Output = Vector3;
    extern "rust-call" fn call_once(self, _: (Vector3, &mut Vector3)) -> Vector3 {
        panic!("FnOnce<(Vector3, &mut Vector3)> not implemented for Transform");
    }
}

impl FnMut<(Vector3, &mut Vector3)> for Transform {
    extern "rust-call" fn call_mut(&mut self, _: (Vector3, &mut Vector3)) -> Vector3 {
        panic!("FnMut<(Vector3, &mut Vector3)> not implemented for Transform");
    }
}

impl Fn<(Vector3, &mut Vector3)> for Transform {
    extern "rust-call" fn call(&self, args: (Vector3, &mut Vector3)) -> Vector3 {
        let (v, abs_error) = args;

        if self.is_identity() {
            *abs_error = Vector3::new(0.0, 0.0, 0.0);
            return v;
        }

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
}

impl FnOnce<(Normal,)> for Transform {
    type Output = Normal;
    extern "rust-call" fn call_once(self, _: (Normal,)) -> Normal {
        panic!("FnOnce<(Normal,)> not implemented for Transform");
    }
}

impl FnMut<(Normal,)> for Transform {
    extern "rust-call" fn call_mut(&mut self, _: (Normal,)) -> Normal {
        panic!("FnMut<(Normal,)> not implemented for Transform");
    }
}

impl Fn<(Normal,)> for Transform {
    extern "rust-call" fn call(&self, n: (Normal,)) -> Normal {
        let n = n.0;
        if self.is_identity() {
            return n;
        }

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
}

impl FnOnce<(Bounds,)> for Transform {
    type Output = Bounds;
    extern "rust-call" fn call_once(self, _: (Bounds,)) -> Bounds {
        panic!("FnOnce<(AABBbounds,)> not implemented for Transform");
    }
}

impl FnMut<(Bounds,)> for Transform {
    extern "rust-call" fn call_mut(&mut self, _: (Bounds,)) -> Bounds {
        panic!("FnMut<(AABBbounds,)> not implemented for Transform");
    }
}

impl Fn<(Bounds,)> for Transform {
    extern "rust-call" fn call(&self, bounds: (Bounds,)) -> Bounds {
        let bounds = bounds.0;

        if self.is_identity() {
            return bounds;
        }

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
}

impl FnOnce<(Ray,)> for Transform {
    type Output = Ray;
    extern "rust-call" fn call_once(self, _: (Ray,)) -> Ray {
        panic!("FnOnce<(Ray,)> not implemented for Transform");
    }
}

impl FnMut<(Ray,)> for Transform {
    extern "rust-call" fn call_mut(&mut self, _: (Ray,)) -> Ray {
        panic!("FnMut<(Ray,)> not implemented for Transform");
    }
}

impl Fn<(Ray,)> for Transform {
    extern "rust-call" fn call(&self, ray: (Ray,)) -> Ray {
        let ray = ray.0;

        if self.is_identity() {
            return ray;
        }

        let mut o_error = Vector3::invalid();
        let o = (self)(ray.o, &mut o_error);
        let d = (self)(ray.d);

        let length_squared = d.length_squared();
        let dt = d.abs().dot(o_error) / length_squared;

        return Ray::new(o + d * dt, d, ray.t_max - dt);
    }
}

impl FnOnce<(Ray, &mut Vector3, &mut Vector3)> for Transform {
    type Output = Ray;
    extern "rust-call" fn call_once(self, _: (Ray, &mut Vector3, &mut Vector3)) -> Ray {
        panic!("FnOnce<(Ray, &mut Vector3, &mut Vector3)> not implemented for Transform");
    }
}

impl FnMut<(Ray, &mut Vector3, &mut Vector3)> for Transform {
    extern "rust-call" fn call_mut(&mut self, _: (Ray, &mut Vector3, &mut Vector3)) -> Ray {
        panic!("FnMut<(Ray, &mut Vector3, &mut Vector3)> not implemented for Transform");
    }
}

impl Fn<(Ray, &mut Vector3, &mut Vector3)> for Transform {
    extern "rust-call" fn call(&self, ray: (Ray, &mut Vector3, &mut Vector3)) -> Ray {
        let (ray, o_error, d_error) = ray;

        if self.is_identity() {
            *o_error = Vector3::new(0.0, 0.0, 0.0);
            *d_error = Vector3::new(0.0, 0.0, 0.0);
            return ray;
        }
        let o = (self)(ray.o, o_error);
        let d = (self)(ray.d, d_error);

        let length_squared = d.length_squared();
        let dt = d.abs().dot(*o_error) / length_squared;

        return Ray::new(o + d * dt, d, ray.t_max);
    }
}

impl FnOnce<(SurfaceInteraction,)> for Transform {
    type Output = SurfaceInteraction;
    extern "rust-call" fn call_once(self, _: (SurfaceInteraction,)) -> SurfaceInteraction {
        panic!("FnOnce<(SurfaceInteraction,)> not implemented for Transform");
    }
}

impl FnMut<(SurfaceInteraction,)> for Transform {
    extern "rust-call" fn call_mut(&mut self, _: (SurfaceInteraction,)) -> SurfaceInteraction {
        panic!("FnMut<(SurfaceInteraction,)> not implemented for Transform");
    }
}

impl Fn<(SurfaceInteraction,)> for Transform {
    extern "rust-call" fn call(&self, args: (SurfaceInteraction,)) -> SurfaceInteraction {
        let (si,) = args;
        let mut ret = SurfaceInteraction::default();
        ret.p = (self)(si.p, si.p_error, &mut ret.p_error);
        ret.n = ((self)(si.n)).normalize();
        ret.entering_material = si.entering_material;
        ret.material = si.material.clone();

        return ret;
    }
}
