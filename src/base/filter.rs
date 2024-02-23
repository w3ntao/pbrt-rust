use crate::pbrt::*;

pub struct FilterSample {
    pub p: Point2f,
    pub weight: f64,
}

pub trait Filter: Send + Sync {
    fn get_integral(&self) -> f64;

    fn sample(&self, u: Point2f) -> FilterSample;

    fn evaluate(&self, p: Point2f) -> f64;
}
