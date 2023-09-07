use crate::pbrt::*;

pub trait Primitive: Send + Sync {
    fn intersect(&self, ray: &dyn Ray, t_max: Float) -> Option<ShapeIntersection>;

    fn fast_intersect(&self, ray: &dyn Ray, t_max: Float) -> bool;

    fn bounds(&self) -> Bounds3f;
}
