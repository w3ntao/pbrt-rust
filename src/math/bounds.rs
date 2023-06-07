use crate::pbrt::*;

pub struct Bounds2f {
    pub p_min: Point2f,
    pub p_max: Point2f,
}

impl Bounds2f {
    pub fn new(points: &[Point2f]) -> Bounds2f {
        let mut _min = points[0];
        let mut _max = points[0];

        for idx in 1..points.len() {
            _min = _min.min(&points[idx]);
            _max = _max.max(&points[idx]);
        }

        return Bounds2f {
            p_min: _min,
            p_max: _max,
        };
    }
}
