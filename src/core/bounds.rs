use crate::core::pbrt::*;

#[derive(Copy, Clone)]
pub struct Bounds {
    pub p_min: Point,
    pub p_max: Point,
}

impl Default for Bounds {
    fn default() -> Self {
        return Bounds::empty();
    }
}

impl Bounds {
    pub fn empty() -> Self {
        return Bounds {
            p_min: Point::invalid(),
            p_max: Point::invalid(),
        };
    }

    pub fn build(points: &[Point]) -> Self {
        let _min = min_of(&points);
        let _max = max_of(&points);
        return Bounds {
            p_min: _min,
            p_max: _max,
        };
    }

    pub fn is_empty(&self) -> bool {
        if self.p_min.is_valid() && self.p_max.is_valid() {
            return false;
        }
        return true;
    }

    pub fn get_area(&self) -> f32 {
        if self.is_empty() {
            return 0.0;
        }
        let x_extent = self.p_max.x - self.p_min.x;
        let y_extent = self.p_max.y - self.p_min.y;
        let z_extent = self.p_max.z - self.p_min.z;

        return 2.0 * (x_extent * y_extent + x_extent * z_extent + y_extent * z_extent);
    }

    pub fn intersect(&self, ray: &Ray) -> (f32, f32) {
        let no_hit = (f32::INFINITY, -f32::INFINITY);
        if self.is_empty() {
            return no_hit;
        }

        let mut t_min = -f32::INFINITY;
        let mut t_max = f32::INFINITY;

        for axis in 0..3 {
            if ray.d[axis] == 0.0 {
                if ray.o[axis] < self.p_min[axis] || ray.o[axis] > self.p_max[axis] {
                    return no_hit;
                }
            } else {
                let mut t1 = (self.p_min[axis] - ray.o[axis]) / ray.d[axis];
                let mut t2 = (self.p_max[axis] - ray.o[axis]) / ray.d[axis];
                if t1 > t2 {
                    mem::swap(&mut t1, &mut t2);
                }
                t_min = t_min.max(t1);
                t_max = t_max.min(t2);
                if t_min > t_max {
                    return no_hit;
                }
            }
        }

        return (t_min, t_max);
    }
}

impl ops::Add<Bounds> for Bounds {
    type Output = Bounds;
    fn add(self, rhs: Bounds) -> Bounds {
        return Bounds {
            p_min: min_of(&[self.p_min, rhs.p_min]),
            p_max: max_of(&[self.p_max, rhs.p_max]),
        };
    }
}

impl ops::AddAssign<Bounds> for Bounds {
    fn add_assign(&mut self, rhs: Bounds) {
        self.p_min = min_of(&[self.p_min, rhs.p_min]);
        self.p_max = max_of(&[self.p_max, rhs.p_max]);
    }
}
