use crate::core::pbrt::*;

#[derive(Clone)]
pub struct SurfaceInteraction {
    pub p: Point,
    pub p_error: Vector3,
    pub n: Normal,
    pub material: Option<Arc<dyn Material>>,
    pub entering_material: bool,
    pub u: f32,
    pub v: f32,
    // uv coordinate is for texture
}

impl Default for SurfaceInteraction {
    fn default() -> Self {
        return Self {
            p: Point::invalid(),
            p_error: Vector3::invalid(),
            n: Normal::invalid(),
            material: None,
            entering_material: true,
            u: f32::NAN,
            v: f32::NAN,
        };
    }
}

impl SurfaceInteraction {
    pub fn offset_ray_origin(&self, w: Vector3) -> Point {
        let d = Vector3::from(self.n).abs().dot(self.p_error);
        let mut offset = d * Vector3::from(self.n);
        if self.n.dot(w) < 0.0 {
            offset = -offset;
        }
        let mut po = self.p + offset;
        // Round offset point _po_ away from _p_

        for idx in 0..3 {
            if offset[idx] > 0.0 {
                po[idx] = next_float_up(po[idx]);
            } else if offset[idx] < 0.0 {
                po[idx] = next_float_down(po[idx]);
            }
        }

        return po;
    }

    pub fn spawn_ray(&self, d: Vector3) -> Ray {
        let o = self.offset_ray_origin(d);
        return Ray::new(o, d, f32::INFINITY);
    }

    const SHADOW_EPSILON: f32 = 0.001;
    pub fn spawn_shadow_ray(&self, target: Point) -> Ray {
        // to shoot a ray from interaction point to the light.
        // this intersection test will fails (because of limited ray.t_max)
        // if the path is reachable

        let d = target - self.p;
        let o = self.offset_ray_origin(d);

        return Ray::new(o, d, 1.0 - SurfaceInteraction::SHADOW_EPSILON);
    }
}
