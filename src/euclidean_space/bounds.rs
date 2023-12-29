use crate::pbrt::*;

pub struct Bounds2<T> {
    pub p_min: Point2<T>,
    pub p_max: Point2<T>,
}

impl Bounds2<i32> {
    pub fn new(points: &[Point2<i32>]) -> Bounds2<i32> {
        let mut _min = points[0];
        let mut _max = points[0];

        for idx in 1..points.len() {
            _min = _min.min(&points[idx]);
            _max = _max.max(&points[idx]);
        }

        return Bounds2::<i32> {
            p_min: _min,
            p_max: _max,
        };
    }
}

impl Bounds2<f64> {
    pub fn new(points: &[Point2<f64>]) -> Bounds2<f64> {
        let mut _min = points[0];
        let mut _max = points[0];

        for idx in 1..points.len() {
            _min = _min.min(&points[idx]);
            _max = _max.max(&points[idx]);
        }

        return Bounds2::<f64> {
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

impl Bounds3<f64> {
    pub fn empty() -> Self {
        return Self {
            p_min: Point3f::new(f64::INFINITY, f64::INFINITY, f64::INFINITY),
            p_max: Point3f::new(f64::NEG_INFINITY, f64::NEG_INFINITY, f64::NEG_INFINITY),
        };
    }

    pub fn from_single_point(p: Point3f) -> Self {
        return Self { p_min: p, p_max: p };
    }

    pub fn from_multiple_points(points: &[Point3f]) -> Self {
        if points.len() < 2 {
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

    pub fn max_dimension(&self) -> usize {
        let d = self.diagonal();
        return if d.x > d.y && d.x > d.z {
            0
        } else if d.y > d.z {
            1
        } else {
            2
        };
    }

    pub fn union(&self, p: Point3f) -> Bounds3<f64> {
        return Bounds3::<f64> {
            p_min: self.p_min.min(p),
            p_max: self.p_max.max(p),
        };
    }

    pub fn diagonal(&self) -> Vector3f {
        return self.p_max - self.p_min;
    }

    pub fn surface_area(&self) -> f64 {
        let d = self.diagonal();
        return 2.0 * (d.x * d.y + d.x * d.z + d.y * d.z);
    }

    pub fn fast_intersect(
        &self,
        ray: &Ray,
        ray_t_max: f64,
        inv_dir: Vector3f,
        dir_is_neg: [usize; 3],
    ) -> bool {
        // Check for ray intersection against $x$ and $y$ slabs
        let o = ray.o;
        let mut t_min = (self[dir_is_neg[0]].x - o.x) * inv_dir.x;
        let mut t_max = (self[1 - dir_is_neg[0]].x - o.x) * inv_dir.x;
        let ty_min = (self[dir_is_neg[1]].y - o.y) * inv_dir.y;
        let mut ty_max = (self[1 - dir_is_neg[1]].y - o.y) * inv_dir.y;

        // Update _tMax_ and _tyMax_ to ensure robust bounds intersection
        t_max *= 1.0 + 2.0 * gamma(3);
        ty_max *= 1.0 + 2.0 * gamma(3);

        if t_min > ty_max || ty_min > t_max {
            return false;
        }

        if ty_min > t_min {
            t_min = ty_min;
        }

        if ty_max < t_max {
            t_max = ty_max;
        }

        // Check for ray intersection against $z$ slab
        let tz_min = (self[dir_is_neg[2]].z - o.z) * inv_dir.z;
        let mut tz_max = (self[1 - dir_is_neg[2]].z - o.z) * inv_dir.z;

        // Update _tzMax_ to ensure robust bounds intersection
        tz_max *= 1.0 + 2.0 * gamma(3);

        if t_min > tz_max || tz_min > t_max {
            return false;
        }

        if tz_min > t_min {
            t_min = tz_min;
        }

        if tz_max < t_max {
            t_max = tz_max;
        }

        return t_min < ray_t_max && t_max > 0.0;
    }
}

impl Add<Bounds3<f64>> for Bounds3<f64> {
    type Output = Bounds3<f64>;

    fn add(self, rhs: Bounds3<f64>) -> Self::Output {
        return Bounds3::<f64> {
            p_min: self.p_min.min(rhs.p_min),
            p_max: self.p_max.max(rhs.p_max),
        };
    }
}

impl AddAssign<Bounds3<f64>> for Bounds3<f64> {
    fn add_assign(&mut self, rhs: Bounds3<f64>) {
        *self = *self + rhs;
    }
}

impl Sum for Bounds3<f64> {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        return match iter.into_iter().reduce(|x, y| x + y) {
            None => Bounds3::<f64>::empty(),
            Some(val) => val,
        };
    }
}
