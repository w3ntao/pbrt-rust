use std::sync::Arc;

use crate::fundamental::point::*;
use crate::fundamental::vector3::*;
use crate::ray_tracing::bounding_box::BoundingBox;
use crate::ray_tracing::intersection::*;
use crate::ray_tracing::material::*;
use crate::ray_tracing::primitive::Primitive;
use crate::ray_tracing::ray::*;

pub struct Sphere {
    pub center: Point,
    pub radius: f32,
    pub material: Arc<dyn Material>,
    bounds: BoundingBox,
}

impl Sphere {
    pub fn new(_center: Point, _radius: f32) -> Self {
        let min = _center + Point::new(-_radius, -_radius, -_radius);
        let max = _center + Point::new(_radius, _radius, _radius);
        return Self {
            center: _center,
            radius: _radius,
            bounds: BoundingBox::build(&[min, max]),
            material: Arc::new(NullMaterial {}),
        };
    }
}

impl Primitive for Sphere {
    fn intersect(&self, ray: &Ray, t_min: f32, t_max: f32) -> Intersection {
        let oc = ray.origin - self.center;
        let a = ray.direction.length_squared();
        let half_b = dot(oc, ray.direction);
        let c = oc.length_squared() - self.radius * self.radius;

        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return Intersection::failure();
        }
        let sqrt_d = discriminant.sqrt();

        let mut root = (-half_b - sqrt_d) / a;
        if root < t_min || root > t_max {
            root = (-half_b + sqrt_d) / a;
            if root < t_min || root > t_max {
                return Intersection::failure();
            }
        }
        let root = root;
        let normal = (ray.get_point(root) - self.center) / self.radius;
        
        return if dot(ray.direction, normal) < 0.0 {
            Intersection::from_outside(root, ray,
                                       normal, self.material.clone())
        } else {
            Intersection::from_inside(root, ray,
                                      -normal, self.material.clone())
        };
    }

    fn get_bounds(&self) -> BoundingBox {
        return self.bounds;
    }

    fn set_material(&mut self, material: Arc<dyn Material>) {
        self.material = material;
    }
}
