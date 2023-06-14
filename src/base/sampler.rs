use crate::pbrt::*;

pub struct SimpleSampler {}

impl SimpleSampler {
    pub fn new() -> Self {
        return SimpleSampler {};
    }

    pub fn get_1d(&self) -> Float {
        return 0.5;
    }

    pub fn get_2d(&self) -> Point2f {
        return Point2f::new(0.5, 0.5);
    }
}
