use crate::core::bounds::Bounds;
use crate::core::intersection::*;
use crate::core::material::Material;
use crate::core::point::*;
use crate::core::primitive::Primitive;
use crate::core::ray::*;
use crate::fundamental::constants::INTERSECT_OFFSET;
use crate::primitives::sphere::*;
use std::sync::Arc;

pub struct HollowSphere {
    pub external_sphere: Sphere,
    pub internal_sphere: Sphere,
}

impl HollowSphere {
    pub fn new(_center: Point, _radius: f32, thickness: f32) -> Self {
        return Self {
            external_sphere: Sphere::new(_center, _radius),
            internal_sphere: Sphere::new(_center, _radius - thickness),
        };
    }
}

impl Primitive for HollowSphere {
    fn intersect(&self, ray: &Ray, t_min: f32, t_max: f32) -> Intersection {
        let external_intersection = self.external_sphere.intersect(ray, t_min, t_max);
        if !external_intersection.intersected() || external_intersection.entering_material {
            return external_intersection;
        }

        let mut internal_intersection =
            self.internal_sphere
                .intersect(ray, INTERSECT_OFFSET, external_intersection.distance);
        if !internal_intersection.intersected() {
            return external_intersection;
        }

        internal_intersection.entering_material = !internal_intersection.entering_material;

        return internal_intersection;
    }

    fn get_bounds(&self) -> Bounds {
        return self.external_sphere.get_bounds();
    }

    fn set_material(&mut self, material: Arc<dyn Material>) {
        self.external_sphere.set_material(material.clone());
        self.internal_sphere.set_material(material);
    }
}
