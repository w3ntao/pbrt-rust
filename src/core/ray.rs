use crate::core::pbrt::*;

#[derive(Copy, Clone)]
pub struct Ray {
    pub o: Point,
    pub d: Vector3,
}

impl Ray {
    pub fn new(_o: Point, _d: Vector3) -> Self {
        return Self { o: _o, d: _d };
    }

    pub fn dummy() -> Self {
        Self {
            o: Point::invalid(),
            d: Vector3::invalid(),
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
