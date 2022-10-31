use crate::core::pbrt::*;

pub const INTERSECT_EPSILON: f32 = 0.001;

#[derive(Clone)]
pub struct SurfaceInteraction {
    pub t: f32,
    pub p: Point,
    pub p_error: Vector3,
    pub n: Normal,
    pub material: Arc<dyn Material>,
    pub entering_material: bool,
    pub u: f32,
    pub v: f32,
    // uv coordinate is for texture
}

impl SurfaceInteraction {
    pub fn new(_t: f32, _p: Point, _n: Normal, _material: Arc<dyn Material>) -> Self {
        return Self {
            t: _t,
            p: _p,
            p_error: Vector3::invalid(),
            n: _n.normalize(),
            material: _material,
            entering_material: true,
            u: f32::NAN,
            v: f32::NAN,
        };
    }

    pub fn new_with_error(
        _t: f32,
        _p: Point,
        _p_error: Vector3,
        _n: Normal,
        _material: Arc<dyn Material>,
    ) -> Self {
        return Self {
            t: _t,
            p: _p,
            p_error: _p_error,
            n: _n.normalize(),
            material: _material,
            entering_material: true,
            u: f32::NAN,
            v: f32::NAN,
        };
    }

    pub fn failure() -> Self {
        return Self {
            t: f32::INFINITY,
            p: Point::invalid(),
            p_error: Vector3::invalid(),
            n: Normal::invalid(),
            material: Arc::new(NullMaterial {}),
            entering_material: true,
            u: f32::NAN,
            v: f32::NAN,
        };
    }
}
