use crate::pbrt::*;

use crate::math::point::Point2;

pub struct Bounds2<T: Numerical> {
    pub p_min: Point2<T>,
    pub p_max: Point2<T>,
}

impl Bounds2<Float> {
    pub fn new(points: &[Point2<Float>]) -> Bounds2<Float> {
        let mut _min = points[0];
        let mut _max = points[0];

        for idx in 1..points.len() {
            _min = _min.min(&points[idx]);
            _max = _max.max(&points[idx]);
        }

        return Bounds2::<Float> {
            p_min: _min,
            p_max: _max,
        };
    }
}

pub type Bounds2f = Bounds2<Float>;
