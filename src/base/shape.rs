use crate::pbrt::*;

pub const MIN_SPHERICAL_SAMPLE_AREA: f64 = 3e-4;
pub const MAX_SPHERICAL_SAMPLE_AREA: f64 = 6.22;

#[derive(Clone)]
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

pub struct ShapeSampleContext {
    pub pi: Point3fi,
    pub n: Normal3f,
    pub ns: Normal3f,
}

pub struct ShapeSample {
    pub interaction: Interaction,
    pub pdf: f64,
}

impl ShapeSampleContext {
    pub fn offset_ray_origin(&self, w: Vector3f) -> Point3f {
        panic!("ShapeSampleContext::offset_ray_origin() not implemented");
    }

    pub fn spawn_ray(&self, w: Vector3f) -> Ray {
        panic!("ShapeSampleContext::spawn_ray() not implemented");
    }
}

#[derive(Clone)]
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

    fn sample(&self, u: Point2f) -> Option<ShapeSample>;

    fn sample_with_context(&self, ctx: &ShapeSampleContext, u: Point2f) -> Option<ShapeSample>;
}
