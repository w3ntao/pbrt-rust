use crate::core::pbrt::*;

#[derive(Copy, Clone)]
pub struct Ray {
    pub o: Point,
    pub d: Vector3,
    pub t_min: f32,
    pub t_max: f32,
}

const RAY_T_MIN: f32 = 0.001;

impl Ray {
    pub fn new(_o: Point, _d: Vector3, _t_min: f32, _t_max: f32) -> Self {
        return Self {
            o: _o,
            d: _d,
            t_min: _t_min,
            t_max: _t_max,
        };
    }

    pub fn update_t(&self, new_t: f32) -> Ray {
        return Ray {
            o: self.o,
            d: self.d,
            t_min: RAY_T_MIN,
            t_max: new_t,
        };
    }

    pub fn dummy() -> Self {
        Self {
            o: Point::invalid(),
            d: Vector3::invalid(),
            t_min: f32::NAN,
            t_max: f32::NAN,
        }
    }
}

// https://docs.rs/fn_ops/latest/fn_ops/
impl FnOnce<(f32,)> for Ray {
    type Output = Point;
    extern "rust-call" fn call_once(self, _: (f32,)) -> Point {
        panic!("FnOnce not implemented for Ray");
    }
}

impl FnMut<(f32,)> for Ray {
    extern "rust-call" fn call_mut(&mut self, _: (f32,)) -> Point {
        panic!("FnMut not implemented for Ray");
    }
}

impl Fn<(f32,)> for Ray {
    extern "rust-call" fn call(&self, args: (f32,)) -> Point {
        return self.o + args.0 * self.d;
    }
}
