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
        for idx in 0..3 {
            self.m[idx][3] += t[idx];
        }
        self.inv_m = self.m.inverse();
    }

    pub fn scale_by_scalar(&mut self, scalar: f32) {
        for row in 0..3 {
            for col in 0..3 {
                self.m[row][col] *= scalar;
            }
        }

        self.inv_m = self.m.inverse();
    }

    pub fn rotate(&mut self, axis: Vector3, angle: f32) {
        let cosine = f32::cos(angle);
        let sine = f32::sin(angle);

        let normalized_axis = axis.normalize();
        let x = normalized_axis.x;
        let y = normalized_axis.y;
        let z = normalized_axis.z;

        let rotate_matrix = Matrix::new(
            Vector4::new(
                x * x * (1.0 - cosine) + cosine,
                x * y * (1.0 - cosine) - z * sine,
                x * z * (1.0 - cosine) + y * sine,
                0.0,
            ),
            Vector4::new(
                x * y * (1.0 - cosine) + z * sine,
                cosine + y * y * (1.0 - cosine),
                y * z * (1.0 - cosine) - x * sine,
                0.0,
            ),
            Vector4::new(
                x * z * (1.0 - cosine) - y * sine,
                y * z * (1.0 - cosine) + x * sine,
                cosine + z * z * (1.0 - cosine),
                0.0,
            ),
            Vector4::new(0.0, 0.0, 0.0, 1.0),
        );

        self.m *= rotate_matrix;
        self.inv_m = self.m.inverse();
    }
}

impl ops::Index<usize> for Transform {
    type Output = Vector4;
    fn index(&self, idx: usize) -> &Vector4 {
        &self.m[idx]
    }
}

// https://docs.rs/fn_ops/latest/fn_ops/
impl FnOnce<(Point,)> for Transform {
    type Output = Point;
    extern "rust-call" fn call_once(self, _: (Point,)) -> Point {
        panic!("FnOnce<(Point,)> not implemented for Transform");
    }
}

impl FnOnce<(Vector3,)> for Transform {
    type Output = Vector3;
    extern "rust-call" fn call_once(self, _: (Vector3,)) -> Vector3 {
        panic!("FnOnce<(Vector3,)> not implemented for Transform");
    }
}

impl FnOnce<(Normal,)> for Transform {
    type Output = Normal;
    extern "rust-call" fn call_once(self, _: (Normal,)) -> Normal {
        panic!("FnOnce<(Normal,)> not implemented for Transform");
    }
}

impl FnOnce<(&Ray,)> for Transform {
    type Output = (Ray, f32);
    extern "rust-call" fn call_once(self, _: (&Ray,)) -> (Ray, f32) {
        panic!("FnOnce<(&Ray,)> not implemented for Transform");
    }
}

impl FnMut<(Point,)> for Transform {
    extern "rust-call" fn call_mut(&mut self, _: (Point,)) -> Point {
        panic!("FnMut<(Point,)> not implemented for Transform");
    }
}

impl FnMut<(Vector3,)> for Transform {
    extern "rust-call" fn call_mut(&mut self, _: (Vector3,)) -> Vector3 {
        panic!("FnMut<(Vector3,)> not implemented for Transform");
    }
}

impl FnMut<(Normal,)> for Transform {
    extern "rust-call" fn call_mut(&mut self, _: (Normal,)) -> Normal {
        panic!("FnMut<(Normal,)> not implemented for Transform");
    }
}

impl FnMut<(&Ray,)> for Transform {
    extern "rust-call" fn call_mut(&mut self, _: (&Ray,)) -> (Ray, f32) {
        panic!("FnMut<(&Ray,)> not implemented for Transform");
    }
}

impl Fn<(Point,)> for Transform {
    extern "rust-call" fn call(&self, p: (Point,)) -> Point {
        let p = p.0;

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

        let pError = gamma(3) * Vector3::new(xAbsSum, yAbsSum, zAbsSum);

        if wp == 1.0 {
            return Point::new(xp, yp, zp);
        }
        return Point::new(xp, yp, zp) / wp;
    }
}

impl Fn<(Vector3,)> for Transform {
    extern "rust-call" fn call(&self, v: (Vector3,)) -> Vector3 {
        let v = v.0;
        return Vector3::from(&self.m * Vector4::from(v));
    }
}

impl Fn<(Normal,)> for Transform {
    extern "rust-call" fn call(&self, n: (Normal,)) -> Normal {
        let n = n.0;
        return Normal::from(&self.inv_m.transpose() * Vector3::from(n)).normalize();
    }
}

impl Fn<(&Ray,)> for Transform {
    extern "rust-call" fn call(&self, ray: (&Ray,)) -> (Ray, f32) {
        let ray = ray.0;

        let inverted_matrix = &self.inv_m;
        let inverted_ray_direction = inverted_matrix * ray.d;
        let inverted_distance = (inverted_matrix * ray.d).length();

        return (
            Ray::new(
                inverted_matrix * ray.o,
                inverted_ray_direction.normalize(),
                ray.t_min / inverted_distance,
                ray.t_max / inverted_distance,
            ),
            inverted_distance,
        );
    }
}
