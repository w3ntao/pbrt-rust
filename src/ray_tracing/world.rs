use std::sync::Arc;

use crate::fundamental::point::Point;
use crate::fundamental::utility::Vector3;
use crate::ray_tracing::group::Group;
use crate::ray_tracing::groups::bvh::BVH;
use crate::ray_tracing::intersection::Intersection;
use crate::ray_tracing::primitive::Primitive;
use crate::ray_tracing::ray::Ray;

pub struct World {
    lights: Vec<Arc<dyn Primitive>>,
    scene: BVH,
}

impl Default for World {
    fn default() -> Self {
        return Self {
            lights: vec![],
            scene: BVH::default(),
        };
    }
}

impl World {
    pub fn add(&mut self, object: Arc<dyn Primitive>) {
        self.scene.add(object.clone());
    }

    pub fn add_light(&mut self, light: Arc<dyn Primitive>) {
        self.lights.push(light.clone());
        self.scene.add(light.clone());
    }

    pub fn build_index(&mut self) {
        self.scene.build_index();
    }

    pub fn intersect(&self, ray: &Ray, t_min: f32, t_max: f32) -> Intersection {
        return self.scene.intersect(ray, t_min, t_max);
    }

    pub fn sample_light(&self) -> (u128, Point, Vector3, f32) {
        // TODO: randomly pick a light
        let idx = 0;
        let (point, normal) = self.lights[idx].sample();

        return (self.lights[idx].get_id(), point, normal, self.lights[idx].get_area());
    }
}
