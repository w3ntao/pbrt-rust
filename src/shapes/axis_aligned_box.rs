use crate::core::pbrt::*;

pub struct AxisAlignedBox {
    pub p_min: Point,
    pub p_max: Point,
    bounds: Bounds,
}

impl AxisAlignedBox {
    pub fn new(corner_0: Point, corner_1: Point) -> Self {
        return Self {
            p_min: min_of(&[corner_0, corner_1]),
            p_max: max_of(&[corner_0, corner_1]),
            bounds: Bounds::build(&[corner_0, corner_1]),
        };
    }
}

impl Shape for AxisAlignedBox {
    fn intersect(&self, ray: &Ray, t_hit: &mut f32, interaction: &mut SurfaceInteraction) -> bool {
        let mut root_in = -f32::INFINITY;
        let mut root_out = f32::INFINITY;
        let mut normal = Normal::invalid();

        for axis in 0..3 {
            if ray.d[axis] == 0.0 {
                if self.p_min[axis] > ray.o[axis] || self.p_max[axis] < ray.o[axis] {
                    return false;
                }
            } else {
                let reverse_dir = 1.0 / ray.d[axis];
                let t0 = (self.p_min[axis] - ray.o[axis]) * reverse_dir;
                let t1 = (self.p_max[axis] - ray.o[axis]) * reverse_dir;

                if t0 > t1 {
                    if root_in < t1 {
                        root_in = t1;
                        normal = Normal::new(0.0, 0.0, 0.0);
                        normal[axis] = 1.0;
                    }
                    root_out = root_out.min(t0);
                } else {
                    if root_in < t0 {
                        root_in = t0;
                        normal = Normal::new(0.0, 0.0, 0.0);
                        normal[axis] = -1.0;
                    }
                    root_out = root_out.min(t1);
                }
                if root_out < root_in {
                    return false;
                }
            }
        }

        if root_in < ray.t_min || root_in > ray.t_max {
            return false;
        }

        *t_hit = root_in;
        interaction.p = ray(root_in);
        interaction.n = normal;

        return true;
    }

    fn get_bounds(&self) -> Bounds {
        return self.bounds;
    }
}
