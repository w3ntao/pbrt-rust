use std::sync::Arc;

use rand::Rng;
use rand::thread_rng;

use crate::fundamental::rgb_color::*;
use crate::fundamental::vector::*;
use crate::ray_tracing::integrator::Integrator;
use crate::ray_tracing::ray::Ray;
use crate::ray_tracing::world::World;

pub struct MonteCarloPathTrace {
    world: Arc<World>,
}

impl MonteCarloPathTrace {
    pub fn new(_world: Arc<World>) -> Self {
        return Self { world: _world };
    }
}

impl MonteCarloPathTrace {
    fn trace(&self, ray: &Ray, depth: i32) -> RGBColor {
        if depth <= 0 {
            return RGBColor::black();
        }

        let intersection = self.world.scene.intersect(ray, f32::INFINITY);
        if intersection.intersected() {
            let mut scattered_ray = Ray::dummy();
            let mut attenuation = RGBColor::black();
            if intersection.material.scatter(&mut attenuation, &mut scattered_ray, &ray, &intersection) {
                return attenuation * self.trace(&scattered_ray, depth - 1);
            }
            // light got absorbed if `scatter()` return false
            return RGBColor::black();
        }

        // shoot into sky
        let unit_direction = ray.direction.normalize();
        let t = 0.5 * (unit_direction.y + 1.0);
        return (1.0 - t) * RGBColor::new(1.0, 1.0, 1.0) + t * RGBColor::new(0.5, 0.7, 1.0);
    }
}

impl Integrator for MonteCarloPathTrace {
    fn get_radiance(&self, ray: &Ray) -> RGBColor {
        return self.trace(ray, 50);
    }
}
