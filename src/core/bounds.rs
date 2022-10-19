use crate::core::interfaces::*;
use std::mem;
use std::ops;

#[derive(Copy, Clone)]
pub struct Bounds {
    pub min: Point,
    pub max: Point,
}

impl Default for Bounds {
    fn default() -> Self {
        return Bounds::empty();
    }
}

impl Bounds {
    pub fn empty() -> Self {
        return Bounds {
            min: Point::invalid(),
            max: Point::invalid(),
        };
    }

    pub fn build(points: &[Point]) -> Self {
        let _min = min_of(&points);
        let _max = max_of(&points);
        return Bounds {
            min: _min,
            max: _max,
        };
    }

    pub fn is_empty(&self) -> bool {
        return !self.min.is_valid() || !self.max.is_valid();
    }

    pub fn get_area(&self) -> f32 {
        if self.is_empty() {
            return 0.0;
        }
        let x_extent = self.max.x - self.min.x;
        let y_extent = self.max.y - self.min.y;
        let z_extent = self.max.z - self.min.z;

        return 2.0 * (x_extent * y_extent + x_extent * z_extent + y_extent * z_extent);
    }

    pub fn intersect(&self, ray: &Ray) -> (f32, f32) {
        let no_intersection = (f32::INFINITY, -f32::INFINITY);
        if self.is_empty() {
            return no_intersection;
        }

        let mut t_min = -f32::INFINITY;
        let mut t_max = f32::INFINITY;

        for axis in 0..3 {
            if ray.d[axis] == 0.0 {
                if ray.o[axis] < self.min[axis] || ray.o[axis] > self.max[axis] {
                    return no_intersection;
                }
            } else {
                let mut t1 = (self.min[axis] - ray.o[axis]) / ray.d[axis];
                let mut t2 = (self.max[axis] - ray.o[axis]) / ray.d[axis];
                if t1 > t2 {
                    mem::swap(&mut t1, &mut t2);
                }
                t_min = t_min.max(t1);
                t_max = t_max.min(t2);
                if t_min > t_max {
                    return no_intersection;
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
            min: min_of(&[self.min, rhs.min]),
            max: max_of(&[self.max, rhs.max]),
        };
    }
}

impl ops::AddAssign<Bounds> for Bounds {
    fn add_assign(&mut self, rhs: Bounds) {
        self.min = min_of(&[self.min, rhs.min]);
        self.max = max_of(&[self.max, rhs.max]);
    }
}
