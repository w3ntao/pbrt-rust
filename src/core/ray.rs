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

    pub fn at(self, t: f32) -> Point {
        return self.o + t * self.d;
    }
}

impl std::fmt::Display for Ray {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "o: {}, d: {},  t_max: {}", self.o, self.d, self.t_max)
    }
}
