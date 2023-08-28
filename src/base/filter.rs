use crate::pbrt::*;

pub struct PixelSample {
    pub p: Point2f,
    pub weight: Float,
}

pub struct FilterSample {
    pub p: Point2f,
    pub weight: Float,
}

pub trait Filter: Send + Sync {
    fn sample(&self, u: Point2f) -> FilterSample;

    fn evaluate(&self, p: Point2f) -> Float;
}
