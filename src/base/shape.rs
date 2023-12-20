use crate::pbrt::*;

pub struct Shading {
    pub n: Normal3f,
    pub dpdu: Vector3f,
    pub dpdv: Vector3f,
    pub dndu: Vector3f,
    pub dndv: Vector3f,
}

impl Shading {
    pub fn nan() -> Self {
        return Self {
            n: Normal3f::nan(),
            dpdu: Vector3::nan(),
            dpdv: Vector3::nan(),
            dndu: Vector3::nan(),
            dndv: Vector3::nan(),
        };
    }
}

pub struct ShapeIntersection {
    pub t_hit: f64,
    pub surface_interaction: SurfaceInteraction,
}

pub struct QuadricIntersection {
    pub t_hit: f64,
    pub p_obj: Point3f,
    pub phi: f64,
}

pub trait Shape: Send + Sync {
    fn intersect(&self, ray: &Ray, t_max: f64) -> Option<ShapeIntersection>;

    fn fast_intersect(&self, ray: &Ray, t_max: f64) -> bool;

    fn bounds(&self) -> Bounds3f;

    fn area(&self) -> f64;
}
