use std::sync::Arc;

use crate::fundamental::point::*;
use crate::fundamental::vector3::*;
use crate::ray_tracing::material::{Material, NullMaterial};
use crate::ray_tracing::ray::*;

#[derive(Clone)]
pub struct Intersection {
    pub distance: f32,
    pub ray: Ray,
    pub normal: Vector3,
    pub material: Arc<dyn Material>,
    pub entering_material: bool,
}

impl Intersection {
    pub fn from_outside(_distance: f32, _ray: &Ray, _normal: Vector3, _material: Arc<dyn Material>) -> Self {
        return Self {
            distance: _distance,
            ray: *_ray,
            normal: _normal.normalize(),
            material: _material,
            entering_material: true,
        };
    }

    pub fn from_inside(_distance: f32, _ray: &Ray, _normal: Vector3, _material: Arc<dyn Material>) -> Self {
        return Self {
            distance: _distance,
            ray: *_ray,
            normal: _normal.normalize(),
            material: _material,
            entering_material: false,
        };
    }

    pub fn failure() -> Self {
        return Self {
            ray: Ray::new(Point::zero(), Vector3::zero()),
            distance: f32::INFINITY,
            normal: Vector3::zero(),
            material: Arc::new(NullMaterial {}),
            entering_material: true,
        };
    }

    pub fn intersected(&self) -> bool {
        return self.distance.is_finite();
    }
}
