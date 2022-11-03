use crate::core::pbrt::*;

pub const INTERSECT_EPSILON: f32 = 0.001;

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
