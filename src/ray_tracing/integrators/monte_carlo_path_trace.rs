use std::sync::Arc;
use crate::fundamental::vector::*;
use crate::fundamental::rgb_color::*;
use crate::ray_tracing::ray::Ray;
use crate::ray_tracing::integrator::Integrator;
use crate::ray_tracing::world::World;

pub struct MonteCarloPathTrace {
    world: Arc<World>,
}

impl MonteCarloPathTrace {
    pub fn new(_world: Arc<World>) -> Self {
        return Self { world: _world };
    }
}

impl Integrator for MonteCarloPathTrace {
    fn get_radiance(&self, ray: &Ray) -> RGBColor {
        let intersect = self.world.scene.intersect(ray, f32::INFINITY);
        if !intersect.intersected() {
            return RGBColor::black();
        }

        let normal = intersect.normal.normalize();
        let grey = 0.0_f32.max(dot(-ray.direction, normal));
        return RGBColor::new(grey, grey, grey);
    }
}
