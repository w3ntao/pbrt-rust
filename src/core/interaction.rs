use crate::core::pbrt::*;

pub const INTERSECT_OFFSET: f32 = 0.001;

#[derive(Clone)]
pub struct SurfaceInteraction {
    pub distance: f32,
    pub hit_point: Point,
    pub normal: Normal,
    pub material: Arc<dyn Material>,
    pub entering_material: bool,
    pub u: f32,
    pub v: f32,
    // uv coordinate is for texture
    pub object_id: u128,
}

impl SurfaceInteraction {
    pub fn from_outside(
        _distance: f32,
        _hit_point: Point,
        _normal: Normal,
        _material: Arc<dyn Material>,
        id: u128,
    ) -> Self {
        return Self {
            distance: _distance,
            hit_point: _hit_point,
            normal: _normal.normalize(),
            material: _material,
            entering_material: true,
            u: f32::NAN,
            v: f32::NAN,
            object_id: id,
        };
    }

    pub fn from_inside(
        _distance: f32,
        _hit_point: Point,
        _normal: Normal,
        _material: Arc<dyn Material>,
    ) -> Self {
        return Self {
            distance: _distance,
            hit_point: _hit_point,
            normal: _normal.normalize(),
            material: _material,
            entering_material: false,
            u: f32::NAN,
            v: f32::NAN,
            object_id: 0,
        };
    }

    pub fn failure() -> Self {
        return Self {
            distance: f32::INFINITY,
            hit_point: Point::invalid(),
            normal: Normal::invalid(),
            material: Arc::new(NullMaterial {}),
            entering_material: true,
            u: f32::NAN,
            v: f32::NAN,
            object_id: 0,
        };
    }

    pub fn intersected(&self) -> bool {
        return self.distance.is_finite();
    }
}
