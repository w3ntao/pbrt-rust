use crate::core::pbrt::*;

#[derive(Copy, Clone)]
pub struct Ray {
    pub o: Point,
    pub d: Vector3,
    pub t_max: f32,
}

impl Ray {
    pub fn new(_o: Point, _d: Vector3, _t_max: f32) -> Self {
        return Self {
            o: _o,
            d: _d,
            t_max: _t_max,
        };
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

impl std::fmt::Display for Ray {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "o: {}, d: {},  t_max: {}", self.o, self.d, self.t_max)
    }
}
