use crate::core::pbrt::*;

pub const INTERSECT_OFFSET: f32 = 0.001;

#[derive(Clone)]
pub struct SurfaceInteraction {
    pub t: f32,
    pub p: Point,
    pub n: Normal,
    pub material: Arc<dyn Material>,
    pub entering_material: bool,
    pub u: f32,
    pub v: f32,
    // uv coordinate is for texture
    pub object_id: u128,
}

impl SurfaceInteraction {
    pub fn new(_t: f32, _p: Point, _n: Normal, _material: Arc<dyn Material>, id: u128) -> Self {
        return Self {
            t: _t,
            p: _p,
            n: _n.normalize(),
            material: _material,
            entering_material: true,
            u: f32::NAN,
            v: f32::NAN,
            object_id: id,
        };
    }

    pub fn failure() -> Self {
        return Self {
            t: f32::INFINITY,
            p: Point::invalid(),
            n: Normal::invalid(),
            material: Arc::new(NullMaterial {}),
            entering_material: true,
            u: f32::NAN,
            v: f32::NAN,
            object_id: 0,
        };
    }

    pub fn intersected(&self) -> bool {
        return self.t.is_finite();
    }
}
