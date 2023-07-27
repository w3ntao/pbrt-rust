use crate::pbrt::*;

pub struct Bounds2<T> {
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

#[derive(Copy, Clone)]
pub struct Bounds3<T> {
    pub p_min: Point3<T>,
    pub p_max: Point3<T>,
}

impl<T> Index<usize> for Bounds3<T> {
    type Output = Point3<T>;

    fn index(&self, index: usize) -> &Self::Output {
        return match index {
            0 => &self.p_min,
            1 => &self.p_max,
            _ => {
                panic!("Bounds3: illegal index: {}", index);
            }
        };
    }
}

impl Bounds3<Float> {
    pub fn from_single_point(p: Point3f) -> Self {
        return Self { p_min: p, p_max: p };
    }

    pub fn from_multiple_points(points: &[Point3f]) -> Self {
        if points.len() <= 2 {
            panic!("expect 2 or more points");
        }

        let mut p_min = points[0];
        let mut p_max = points[0];

        for idx in 1..points.len() {
            p_min = p_min.min(points[idx]);
            p_max = p_max.max(points[idx]);
        }

        return Self { p_min, p_max };
    }

    pub fn union(&self, rhs: &Bounds3<Float>) -> Bounds3<Float> {
        return Bounds3::<Float> {
            p_min: self.p_min.min(rhs.p_min),
            p_max: self.p_max.max(rhs.p_max),
        };
    }
}
