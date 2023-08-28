use crate::pbrt::*;

pub trait Primitive: Send + Sync {
    fn intersect(&self, ray: &Ray, t_max: Float) -> Option<ShapeIntersection>;

    fn bounds(&self) -> Bounds3f;
}
