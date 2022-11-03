use crate::core::pbrt::*;

#[derive(Copy, Clone)]
pub struct AABBbounds {
    pub min: Point,
    pub max: Point,
}

impl AABBbounds {
    pub fn empty() -> Self {
        return AABBbounds {
            min: Point::new(f32::INFINITY, f32::INFINITY, f32::INFINITY),
            max: Point::new(-f32::INFINITY, -f32::INFINITY, -f32::INFINITY),
        };
    }

    pub fn build(points: &[Point]) -> Self {
        let _min = min_of(&points);
        let _max = max_of(&points);
        return AABBbounds {
            min: _min,
            max: _max,
        };
    }

    pub fn is_empty(&self) -> bool {
        for idx in 0..3 {
            if self.min[idx] > self.max[idx] {
                return true;
            }
        }
        return false;
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
        let no_hit = (f32::INFINITY, -f32::INFINITY);
        if self.is_empty() {
            return no_hit;
        }

        let mut t0 = -f32::INFINITY;
        let mut t1 = f32::INFINITY;

        for axis in 0..3 {
            if ray.d[axis] == 0.0 {
                if ray.o[axis] < self.min[axis] || ray.o[axis] > self.max[axis] {
                    return no_hit;
                }
            } else {
                let inv_ray_dir = 1.0 / ray.d[axis];
                let mut t_near = (self.min[axis] - ray.o[axis]) * inv_ray_dir;
                let mut t_far = (self.max[axis] - ray.o[axis]) * inv_ray_dir;
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

impl ops::Add<AABBbounds> for AABBbounds {
    type Output = AABBbounds;
    fn add(self, rhs: AABBbounds) -> AABBbounds {
        return AABBbounds {
            min: min_of(&[self.min, rhs.min]),
            max: max_of(&[self.max, rhs.max]),
        };
    }
}

impl ops::AddAssign<AABBbounds> for AABBbounds {
    fn add_assign(&mut self, rhs: AABBbounds) {
        *self = *self + rhs;
    }
}
