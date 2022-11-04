use crate::core::pbrt::*;

pub const INTERSECT_EPSILON: f32 = 0.001;
pub const SHADOW_EPSILON: f32 = 0.001;

#[derive(Clone)]
pub struct SurfaceInteraction {
    pub t: f32,
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
            t: f32::INFINITY,
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

pub fn offset_ray_origin(p: Point, p_error: Vector3, n: Normal, w: Vector3) -> Point {
    let d = Vector3::from(n).abs().dot(p_error);
    let mut offset = d * Vector3::from(n);
    if n.dot(w) < 0.0 {
        offset = -offset;
    }
    let mut po = p + offset;
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

impl SurfaceInteraction {
    pub fn spawn_ray(&self, d: Vector3) -> Ray {
        let o = offset_ray_origin(self.p, self.p_error, self.n, d);
        return Ray::new(o, d, 0.0, f32::INFINITY);
    }

    pub fn spawn_ray_to(&self, target: Point) -> Ray {
        let d = target - self.p;
        let o = offset_ray_origin(self.p, self.p_error, self.n, d);
        return Ray::new(o, d, 0.0, 1.0 - SHADOW_EPSILON);
    }
}
