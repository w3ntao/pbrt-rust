use crate::vector::*;
use crate::ray::*;
use crate::intersection::*;
use crate::primitive::Primitive;

#[derive(Copy, Clone)]
pub struct Triangle {
    pub origin: Vector,
    pub span0: Vector,
    pub span1: Vector,
    pub normal: Vector,
}

impl Triangle {
    pub fn new(v0: Vector, v1: Vector, v2: Vector) -> Self {
        let _span0 = v1 - v0;
        let _span1 = v2 - v0;
        return Self {
            origin: v0,
            span0: _span0,
            span1: _span1,
            normal: cross(_span0, _span1).normalize(),
        };
    }
}

impl Primitive for Triangle {
    fn intersect(&self, ray: &Ray, previous_distance: f32) -> Intersection {
        let ab = cross(self.span0, self.span1);
        let det = -dot(ab, ray.direction);
        if det == 0.0 {
            return Intersection::failure();
        }

        let c = ray.origin - self.origin;
        let t = dot(ab, c) / det;
        if t < 0.0 || t > previous_distance {
            return Intersection::failure();
        }

        let beta = dot(c, cross(ray.direction, self.span1)) / det;
        let gamma = dot(self.span0, cross(ray.direction, c)) / det;
        if beta < 0.0 || gamma < 0.0 || beta + gamma > 1.0 {
            // if the intersection is outside of the triangle
            return Intersection::failure();
        }

        return Intersection::new(t, &ray, self.normal);
    }
}
