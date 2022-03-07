use std::sync::Arc;

use crate::fundamental::point::*;
use crate::fundamental::vector3::*;
use crate::ray_tracing::bounding_box::BoundingBox;
use crate::ray_tracing::intersection::*;
use crate::ray_tracing::materials::null::NullMaterial;
use crate::ray_tracing::primitive::Primitive;
use crate::ray_tracing::ray::*;

#[derive(Copy, Clone)]
pub struct AxisAlignedBox {
    pub axis_min: Point,
    pub axis_max: Point,
    bounds: BoundingBox,
}

impl AxisAlignedBox {
    pub fn new(corner_0: Point, corner_1: Point) -> Self {
        return Self {
            axis_min: min_of(&[corner_0, corner_1]),
            axis_max: max_of(&[corner_0, corner_1]),
            bounds: BoundingBox::build(&[corner_0, corner_1]),
        };
    }
}

impl Primitive for AxisAlignedBox {
    fn intersect(&self, ray: &Ray, t_max: f32) -> Intersection {
        let mut root_min = 0.0;
        let mut root_max = t_max;
        let mut normal = Vector3::zero();

        for axis in 0..3 {
            if ray.direction[axis] == 0.0 {
                if self.axis_min[axis] > ray.origin[axis] || self.axis_max[axis] < ray.origin[axis] {
                    return Intersection::failure();
                }
            } else {
                let t0 = (self.axis_min[axis] - ray.origin[axis]) / ray.direction[axis];
                let t1 = (self.axis_max[axis] - ray.origin[axis]) / ray.direction[axis];

                if t0 > t1 {
                    if root_min < t1 {
                        root_min = t1;
                        normal = Vector3::new(0.0, 0.0, 0.0);
                        normal[axis] = 1.0;
                    }
                    root_max = root_max.min(t0);
                } else {
                    if root_min < t0 {
                        root_min = t0;
                        normal = Vector3::new(0.0, 0.0, 0.0);
                        normal[axis] = -1.0;
                    }
                    root_max = root_max.min(t1);
                }
                if root_max < root_min {
                    return Intersection::failure();
                }
            }
        }

        return Intersection::new(root_min, ray, normal, Arc::new(NullMaterial {}));
    }

    fn get_bounds(&self) -> BoundingBox {
        return self.bounds;
    }
}
