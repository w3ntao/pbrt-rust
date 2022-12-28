use crate::core::pbrt::*;

#[derive(Copy, Clone)]
pub struct Bounds {
    pub p_min: Point,
    pub p_max: Point,
}

impl Bounds {
    pub fn empty() -> Self {
        return Bounds {
            p_min: Point::new(f32::INFINITY, f32::INFINITY, f32::INFINITY),
            p_max: Point::new(-f32::INFINITY, -f32::INFINITY, -f32::INFINITY),
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
        for idx in 0..3 {
            if self.p_min[idx] > self.p_max[idx] {
                return true;
            }
        }
        return false;
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

        let mut t0 = -f32::INFINITY;
        let mut t1 = f32::INFINITY;

        for axis in 0..3 {
            if ray.d[axis] == 0.0 {
                if ray.o[axis] < self.p_min[axis] || ray.o[axis] > self.p_max[axis] {
                    return no_hit;
                }
            } else {
                let inv_ray_dir = 1.0 / ray.d[axis];
                let mut t_near = (self.p_min[axis] - ray.o[axis]) * inv_ray_dir;
                let mut t_far = (self.p_max[axis] - ray.o[axis]) * inv_ray_dir;
                if t_near > t_far {
                    mem::swap(&mut t_near, &mut t_far);
                }

                t_far *= 1.0 + 2.0 * gamma(3);
                // PBRT: managing rounding error
                // https://www.pbr-book.org/3ed-2018/Shapes/Managing_Rounding_Error#ConservativeRayndashBoundsIntersections

                t0 = t0.max(t_near);
                t1 = t1.min(t_far);
                if t0 > t1 {
                    return no_hit;
                }
            }
        }

        return (t0, t1);
    }
}

impl Sum for Bounds {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        return match iter.into_iter().reduce(|x, y| x + y) {
            None => Bounds::empty(),
            Some(val) => val,
        };
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
        *self = *self + rhs;
    }
}
