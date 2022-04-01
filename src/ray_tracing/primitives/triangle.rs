use std::sync::Arc;

use crate::fundamental::point::*;
use crate::fundamental::vector3::*;
use crate::ray_tracing::bounding_box::BoundingBox;
use crate::ray_tracing::intersection::*;
use crate::ray_tracing::material::Material;
use crate::ray_tracing::material::NullMaterial;
use crate::ray_tracing::primitive::Primitive;
use crate::ray_tracing::ray::*;

pub struct Triangle {
    pub origin: Point,
    pub span0: Vector3,
    pub span1: Vector3,
    pub normal: Vector3,
    bounds: BoundingBox,
    material: Arc<dyn Material>,
}

impl Triangle {
    pub fn new(v0: Point, v1: Point, v2: Point) -> Self {
        let _span0 = v1 - v0;
        let _span1 = v2 - v0;
        return Self {
            origin: v0,
            span0: _span0,
            span1: _span1,
            normal: cross(_span0, _span1).normalize(),
            bounds: BoundingBox::build(&[v0, v1, v2]),
            material: Arc::new(NullMaterial {}),
        };
    }
}

impl Primitive for Triangle {
    fn intersect(&self, ray: &Ray, t_min: f32, t_max: f32) -> Intersection {
        let ab = cross(self.span0, self.span1);
        let det = -dot(ab, ray.direction);
        if det == 0.0 {
            return Intersection::failure();
        }

        let c = ray.origin - self.origin;
        let t = dot(ab, c) / det;
        if t < t_min || t > t_max {
            return Intersection::failure();
        }

        let beta = dot(c, cross(ray.direction, self.span1)) / det;
        let gamma = dot(self.span0, cross(ray.direction, c)) / det;
        let error_tolerance = 0.01;
        // to tolerate numerical error
        if beta < -error_tolerance || gamma < -error_tolerance ||
            beta + gamma > 1.0 + error_tolerance {
            // if the intersection is outside of the triangle
            return Intersection::failure();
        }

        let cos = cosine(ray.direction, self.normal);
        let normal = if cos < 0.0 { self.normal } else { -self.normal };

        return Intersection::from_outside(t, ray.get_point(t), normal, self.material.clone());
    }

    fn get_bounds(&self) -> BoundingBox {
        return self.bounds;
    }

    fn set_material(&mut self, material: Arc<dyn Material>) {
        self.material = material;
    }
}
