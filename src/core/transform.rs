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

    pub fn reset(&mut self) {
        *self = Transform::identity();
    }

    pub fn translate(&mut self, t: Vector3) {
        self.m.translate(t);
        self.inv_m = self.m.inverse();
    }

    pub fn scale_by_scalar(&mut self, scalar: f32) {
        self.m.scale_by_scalar(scalar);
        self.inv_m = self.m.inverse();
    }

    pub fn rotate(&mut self, axis: Vector3, angle: f32) {
        self.m.rotate(axis, angle);
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
        return Point::from(&self.m * Vector4::from(p));
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
        let inverted_length = (inverted_matrix * ray.d).length();

        return (
            Ray::new(inverted_matrix * ray.o, inverted_ray_direction.normalize()),
            inverted_length,
        );
    }
}
