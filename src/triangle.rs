use crate::vector::*;

#[derive(Copy, Clone)]
pub struct Triangle {
    pub origin: Vector,
    pub span0: Vector,
    pub span1: Vector,
    pub normal: Vector,
}

impl Triangle {
    pub fn new(v0: Vector, v1: Vector, v2: Vector) -> Self {
        let _span0 = v1 - v0;
        let _span1 = v2 - v0;
        return Self {
            origin: v0,
            span0: _span0,
            span1: _span1,
            normal: cross(_span0, _span1).normalize(),
        };
    }
}
