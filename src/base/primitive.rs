use crate::pbrt::*;

pub trait Primitive: Send + Sync {
    fn intersect(&self, ray: &Ray, t_max: f64) -> Option<ShapeIntersection>;

    fn fast_intersect(&self, ray: &Ray, t_max: f64) -> bool;

    fn bounds(&self) -> Bounds3f;
}
