use std::sync::Arc;

use crate::fundamental::point::*;
use crate::fundamental::vector3::*;
use crate::ray_tracing::bounding_box::BoundingBox;
use crate::ray_tracing::intersection::*;
use crate::ray_tracing::material::Material;
use crate::ray_tracing::materials::null::NullMaterial;
use crate::ray_tracing::primitive::Primitive;
use crate::ray_tracing::ray::*;

pub struct Sphere {
    pub center: Point,
    pub radius: f32,
    pub material: Arc<dyn Material>,
    bounds: BoundingBox,
}

impl Sphere {
    pub fn new(_center: Point, _radius: f32, _material: Arc<dyn Material>) -> Self {
        let min = _center + Point::new(-_radius, -_radius, -_radius);
        let max = _center + Point::new(_radius, _radius, _radius);
        return Self {
            center: _center,
            radius: _radius,
            bounds: BoundingBox::build(&[min, max]),
            material: _material,
        };
    }
}

impl Primitive for Sphere {
    fn intersect(&self, ray: &Ray, previous_distance: f32) -> Intersection {
        let oc = self.center - ray.origin;
        let dt = dot(oc, ray.direction);
        let discriminant = dt * dt + self.radius * self.radius - dot(oc, oc);

        if discriminant < 0.0 {
            return Intersection::failure();
        }

        let d_sqrt = discriminant.sqrt();
        let t1 = dt - d_sqrt;

        if t1 > previous_distance {
            // previous intersection was closer
            return Intersection::failure();
        }

        if t1 > 0.0 {
            let normal = (ray.get_point(t1) - self.center) / self.radius;

            return {
                if dot(normal, ray.direction) > 0.0 {
                    Intersection::from_inside(t1, ray, normal,
                                              self.material.clone())
                } else {
                    Intersection::new(t1, ray, normal,
                                      self.material.clone())
                }
            };
        }

        let t2 = dt + d_sqrt;
        if t2 < 0.0 || t2 > previous_distance {
            // either the intersection is farther than the closest one, or behind
            return Intersection::failure();
        }

        let normal = (ray.get_point(t2) - self.center) / self.radius;
        return {
            if dot(normal, ray.direction) > 0.0 {
                Intersection::from_inside(t1, ray, normal,
                                          self.material.clone())
            } else {
                Intersection::new(t2, ray, normal,
                                  self.material.clone())
            }
        };
    }

    fn get_bounds(&self) -> BoundingBox {
        return self.bounds;
    }
}
