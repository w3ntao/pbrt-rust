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
    pub fn empty() -> Self {
        return Self {
            p_min: Point3f::new(Float::INFINITY, Float::INFINITY, Float::INFINITY),
            p_max: Point3f::new(
                Float::NEG_INFINITY,
                Float::NEG_INFINITY,
                Float::NEG_INFINITY,
            ),
        };
    }

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

    pub fn union(&self, p: Point3f) -> Bounds3<Float> {
        return Bounds3::<Float> {
            p_min: self.p_min.min(p),
            p_max: self.p_max.max(p),
        };
    }

    pub fn diagonal(&self) -> Vector3f {
        return self.p_max - self.p_min;
    }

    pub fn surface_area(&self) -> Float {
        let d = self.diagonal();
        return 2.0 * (d.x * d.y + d.x * d.z + d.y * d.z);
    }

    pub fn fast_intersect(
        &self,
        ray: &Ray,
        raytMax: Float,
        invDir: Vector3f,
        dirIsNeg: [usize; 3],
    ) -> bool {
        // Check for ray intersection against $x$ and $y$ slabs
        let o = ray.o;
        let d = ray.d;
        let mut tMin = (self[dirIsNeg[0]].x - o.x) * invDir.x;
        let mut tMax = (self[1 - dirIsNeg[0]].x - o.x) * invDir.x;
        let tyMin = (self[dirIsNeg[1]].y - o.y) * invDir.y;
        let mut tyMax = (self[1 - dirIsNeg[1]].y - o.y) * invDir.y;

        // Update _tMax_ and _tyMax_ to ensure robust bounds intersection
        tMax *= 1.0 + 2.0 * gamma(3);
        tyMax *= 1.0 + 2.0 * gamma(3);

        if tMin > tyMax || tyMin > tMax {
            return false;
        }

        if tyMin > tMin {
            tMin = tyMin;
        }

        if tyMax < tMax {
            tMax = tyMax;
        }

        // Check for ray intersection against $z$ slab
        let tzMin = (self[dirIsNeg[2]].z - o.z) * invDir.z;
        let mut tzMax = (self[1 - dirIsNeg[2]].z - o.z) * invDir.z;

        // Update _tzMax_ to ensure robust bounds intersection
        tzMax *= 1.0 + 2.0 * gamma(3);

        if tMin > tzMax || tzMin > tMax {
            return false;
        }

        if tzMin > tMin {
            tMin = tzMin;
        }

        if tzMax < tMax {
            tMax = tzMax;
        }

        return tMin < raytMax && tMax > 0.0;
    }
}

impl Add<Bounds3<Float>> for Bounds3<Float> {
    type Output = Bounds3<Float>;

    fn add(self, rhs: Bounds3<Float>) -> Self::Output {
        return Bounds3::<Float> {
            p_min: self.p_min.min(rhs.p_min),
            p_max: self.p_max.max(rhs.p_max),
        };
    }
}

impl AddAssign<Bounds3<Float>> for Bounds3<Float> {
    fn add_assign(&mut self, rhs: Bounds3<Float>) {
        *self = *self + rhs;
    }
}

impl Sum for Bounds3<Float> {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        return match iter.into_iter().reduce(|x, y| x + y) {
            None => Bounds3::<Float>::empty(),
            Some(val) => val,
        };
    }
}
