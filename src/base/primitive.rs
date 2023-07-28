use crate::pbrt::*;

pub trait Primitive {
    fn intersect(&self, ray: &Ray, t_max: Float) -> Option<ShapeIntersection>;

    fn get_bounds(&self) -> Bounds3f;
}