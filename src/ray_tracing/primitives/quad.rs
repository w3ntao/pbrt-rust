use std::sync::Arc;

use crate::fundamental::point::*;
use crate::fundamental::vector3::*;
use crate::ray_tracing::bounding_box::BoundingBox;
use crate::ray_tracing::intersection::*;
use crate::ray_tracing::materials::null::NullMaterial;
use crate::ray_tracing::primitive::Primitive;
use crate::ray_tracing::ray::*;

#[derive(Copy, Clone)]
pub struct Quad {
    pub origin: Point,
    pub span0: Vector3,
    pub span1: Vector3,
    pub normal: Vector3,
    bounds: BoundingBox,
}

impl Quad {
    pub fn new(v0: Point, _span0: Vector3, _span1: Vector3) -> Self {
        return Self {
            origin: v0,
            span0: _span0,
            span1: _span1,
            normal: cross(_span0, _span1).normalize(),
            bounds: BoundingBox::build(&[v0, v0 + _span0, v0 + _span1, v0 + _span0 + _span1]),
        };
    }
}

impl Primitive for Quad {
    fn intersect(&self, ray: &Ray, previous_distance: f32) -> Intersection {
        let ab = cross(self.span0, self.span1);
        let det = -dot(&ab, &ray.direction);
        if det == 0.0 {
            return Intersection::failure();
        }

        let c = ray.origin - self.origin;
        let det_t = dot(&ab, &c);
        let t = det_t / det;
        if t < 0.0 || t > previous_distance {
            return Intersection::failure();
        }
        let beta = dot(&c, &cross(ray.direction, self.span1)) / det;
        let gamma = dot(&self.span0, &cross(ray.direction, c)) / det;
        if beta < 0.0 || beta > 1.0 || gamma < 0.0 || gamma > 1.0 {
            return Intersection::failure();
        }

        return Intersection::new(t, ray, ab.normalize(), Arc::new(NullMaterial {}));
    }

    fn get_bounds(&self) -> BoundingBox {
        return self.bounds;
    }
}
