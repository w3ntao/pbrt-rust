use std::sync::Arc;
use crate::fundamental::point::*;
use crate::fundamental::vector::*;
use crate::ray_tracing::ray::*;
use crate::ray_tracing::material::Material;
use crate::ray_tracing::materials::null::*;

#[derive(Clone)]
pub struct Intersection {
    pub distance: f32,
    pub ray: Ray,
    pub normal: Vector,
    pub material: Arc<dyn Material>,
}

impl Intersection {
    pub fn new(_distance: f32, _ray: &Ray, _normal: Vector, _material: Arc<dyn Material>) -> Self {
        return Self {
            distance: _distance,
            ray: _ray.clone(),
            normal: _normal,
            material: _material,
        };
    }

    pub fn failure() -> Self {
        return Self {
            ray: Ray::new(Point::zero(), Vector::zero()),
            distance: f32::INFINITY,
            normal: Vector::zero(),
            material: Arc::new(NullMaterial {}),
        };
    }

    pub fn intersected(&self) -> bool {
        return self.distance.is_finite();
    }
}
