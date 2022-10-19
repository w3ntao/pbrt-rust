use crate::core::bounding_box::BoundingBox;
use crate::core::intersection::*;
use crate::core::material::*;
use crate::core::primitive::Primitive;
use crate::core::ray::*;
use crate::fundamental::point::*;
use crate::fundamental::random::{random_f32, random_u128};
use crate::fundamental::vector3::*;
use std::sync::Arc;

pub struct Quad {
    pub origin: Point,
    pub span0: Vector3,
    pub span1: Vector3,
    pub normal: Vector3,
    bounds: BoundingBox,
    material: Arc<dyn Material>,
    id: u128,
}

impl Quad {
    pub fn new(v0: Point, _span0: Vector3, _span1: Vector3) -> Self {
        return Self {
            origin: v0,
            span0: _span0,
            span1: _span1,
            normal: cross(_span0, _span1).normalize(),
            bounds: BoundingBox::build(&[v0, v0 + _span0, v0 + _span1, v0 + _span0 + _span1]),
            material: Arc::new(NullMaterial {}),
            id: random_u128(),
        };
    }
}

impl Primitive for Quad {
    fn intersect(&self, ray: &Ray, t_min: f32, t_max: f32) -> Intersection {
        let ab = cross(self.span0, self.span1);
        let det = -dot(ab, ray.d);
        if det == 0.0 {
            return Intersection::failure();
        }

        let c = ray.o - self.origin;
        let det_t = dot(ab, c);
        let t = det_t / det;
        if t < t_min || t > t_max {
            return Intersection::failure();
        }
        let beta = dot(c, cross(ray.d, self.span1)) / det;
        let gamma = dot(self.span0, cross(ray.d, c)) / det;
        if beta < 0.0 || beta > 1.0 || gamma < 0.0 || gamma > 1.0 {
            return Intersection::failure();
        }

        let normal = if dot(ray.d, self.normal) < 0.0 {
            self.normal
        } else {
            -self.normal
        };
        return Intersection::from_outside(
            t,
            ray.get_point(t),
            normal,
            self.material.clone(),
            self.get_id(),
        );
    }

    fn get_bounds(&self) -> BoundingBox {
        return self.bounds;
    }

    fn set_material(&mut self, material: Arc<dyn Material>) {
        self.material = material;
    }

    fn sample(&self) -> (Point, Vector3) {
        let alpha = random_f32(0.0, 1.0);
        let beta = random_f32(0.0, 1.0);
        return (
            self.origin + alpha * self.span0 + beta * self.span1,
            self.normal,
        );
    }

    fn get_id(&self) -> u128 {
        return self.id;
    }

    fn get_area(&self) -> f32 {
        return cross(self.span0, self.span1).length();
    }
}
