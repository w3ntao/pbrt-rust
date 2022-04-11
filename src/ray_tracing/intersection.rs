use std::sync::Arc;

use rand_distr::num_traits::Float;

use crate::fundamental::point::*;
use crate::fundamental::vector3::*;
use crate::ray_tracing::material::{Material, NullMaterial};

#[derive(Clone)]
pub struct Intersection {
    pub distance: f32,
    pub hit_point: Point,
    pub normal: Vector3,
    pub material: Arc<dyn Material>,
    pub entering_material: bool,
    pub u: f32,
    pub v: f32,
    // uv coordinate is for texture
}

impl Intersection {
    pub fn from_outside(_distance: f32, _hit_point: Point, _normal: Vector3, _material: Arc<dyn Material>) -> Self {
        return Self {
            distance: _distance,
            hit_point: _hit_point,
            normal: _normal.normalize(),
            material: _material,
            entering_material: true,
            u: f32::nan(),
            v: f32::nan(),
        };
    }

    pub fn from_inside(_distance: f32, _hit_point: Point, _normal: Vector3, _material: Arc<dyn Material>) -> Self {
        return Self {
            distance: _distance,
            hit_point: _hit_point,
            normal: _normal.normalize(),
            material: _material,
            entering_material: false,
            u: f32::nan(),
            v: f32::nan(),
        };
    }

    pub fn failure() -> Self {
        return Self {
            distance: f32::INFINITY,
            hit_point: Point::invalid(),
            normal: Vector3::invalid(),
            material: Arc::new(NullMaterial {}),
            entering_material: true,
            u: f32::nan(),
            v: f32::nan(),
        };
    }

    pub fn intersected(&self) -> bool {
        return self.distance.is_finite();
    }
}
