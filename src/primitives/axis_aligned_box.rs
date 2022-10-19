use crate::core::interfaces::*;
use std::sync::Arc;

pub struct AxisAlignedBox {
    pub axis_min: Point,
    pub axis_max: Point,
    pub id: u128,
    bounds: Bounds,
    material: Arc<dyn Material>,
}

impl AxisAlignedBox {
    pub fn new(corner_0: Point, corner_1: Point) -> Self {
        return Self {
            axis_min: min_of(&[corner_0, corner_1]),
            axis_max: max_of(&[corner_0, corner_1]),
            id: random_u128(),
            bounds: Bounds::build(&[corner_0, corner_1]),
            material: Arc::new(NullMaterial {}),
        };
    }
}

impl Primitive for AxisAlignedBox {
    fn intersect(&self, ray: &Ray, t_min: f32, t_max: f32) -> Intersection {
        let mut root_in = -f32::INFINITY;
        let mut root_out = f32::INFINITY;
        let mut normal = Vector3::invalid();

        for axis in 0..3 {
            if ray.d[axis] == 0.0 {
                if self.axis_min[axis] > ray.o[axis] || self.axis_max[axis] < ray.o[axis] {
                    return Intersection::failure();
                }
            } else {
                let t0 = (self.axis_min[axis] - ray.o[axis]) / ray.d[axis];
                let t1 = (self.axis_max[axis] - ray.o[axis]) / ray.d[axis];

                if t0 > t1 {
                    if root_in < t1 {
                        root_in = t1;
                        normal = Vector3::new(0.0, 0.0, 0.0);
                        normal[axis] = 1.0;
                    }
                    root_out = root_out.min(t0);
                } else {
                    if root_in < t0 {
                        root_in = t0;
                        normal = Vector3::new(0.0, 0.0, 0.0);
                        normal[axis] = -1.0;
                    }
                    root_out = root_out.min(t1);
                }
                if root_out < root_in {
                    return Intersection::failure();
                }
            }
        }

        if root_in < t_min || root_in > t_max {
            return Intersection::failure();
        }

        return Intersection::from_outside(
            root_in,
            ray.get_point(root_in),
            normal,
            self.material.clone(),
            self.get_id(),
        );
    }

    fn get_bounds(&self) -> Bounds {
        return self.bounds;
    }

    fn set_material(&mut self, material: Arc<dyn Material>) {
        self.material = material;
    }

    fn get_id(&self) -> u128 {
        return self.id;
    }
}
