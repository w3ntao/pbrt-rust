use std::sync::Arc;

use crate::fundamental::point::*;
use crate::fundamental::utility::{random_u128, random_zero_to_one};
use crate::fundamental::vector3::*;
use crate::ray_tracing::bounding_box::BoundingBox;
use crate::ray_tracing::intersection::*;
use crate::ray_tracing::material::*;
use crate::ray_tracing::primitive::Primitive;
use crate::ray_tracing::ray::*;

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
        let det = -dot(ab, ray.direction);
        if det == 0.0 {
            return Intersection::failure();
        }

        let c = ray.origin - self.origin;
        let det_t = dot(ab, c);
        let t = det_t / det;
        if t < t_min || t > t_max {
            return Intersection::failure();
        }
        let beta = dot(c, cross(ray.direction, self.span1)) / det;
        let gamma = dot(self.span0, cross(ray.direction, c)) / det;
        if beta < 0.0 || beta > 1.0 || gamma < 0.0 || gamma > 1.0 {
            return Intersection::failure();
        }

        let normal = if dot(ray.direction, self.normal) < 0.0 { self.normal } else { -self.normal };
        return Intersection::from_outside(t, ray.get_point(t), normal, self.material.clone(), self.get_id());
    }

    fn get_bounds(&self) -> BoundingBox {
        return self.bounds;
    }

    fn set_material(&mut self, material: Arc<dyn Material>) {
        self.material = material;
    }

    fn sample(&self) -> (Point, Vector3) {
        let alpha = random_zero_to_one();
        let beta = random_zero_to_one();
        return (self.origin + alpha * self.span0 + beta * self.span1, self.normal);
    }

    fn get_id(&self) -> u128 {
        return self.id;
    }

    fn get_area(&self) -> f32 {
        return cross(self.span0, self.span1).length();
    }
}
