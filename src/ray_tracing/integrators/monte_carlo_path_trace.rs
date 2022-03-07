use std::sync::Arc;

use rand::Rng;
use rand::thread_rng;

use crate::fundamental::color::*;
use crate::fundamental::vector3::*;
use crate::ray_tracing::integrator::Integrator;
use crate::ray_tracing::ray::Ray;
use crate::ray_tracing::world::World;

const EPSILON_BLACK: f32 = 1e-4;

pub struct MonteCarloPathTrace {
    world: Arc<World>,
}

impl MonteCarloPathTrace {
    pub fn new(_world: Arc<World>) -> Self {
        return Self { world: _world };
    }
}

impl MonteCarloPathTrace {
    fn trace(&self, ray: &Ray, depth: i32) -> Color {
        if depth <= 0 {
            return Color::black();
        }

        let intersection = self.world.scene.intersect(ray, 0.0, f32::INFINITY);
        if intersection.intersected() {
            let mut scattered_ray = Ray::dummy();
            let attenuation = intersection.material.scatter(&mut scattered_ray, &ray, &intersection);
            if attenuation.r <= EPSILON_BLACK && attenuation.g <= EPSILON_BLACK && attenuation.b <= EPSILON_BLACK {
                // the attenuation goes too low that it probably contribute nothing
                // to the result so we stop here
                // TODO: introduce Russian Roulette in the future to fix this bias
                return attenuation;
            }
            return attenuation * self.trace(&scattered_ray, depth - 1);
        }

        // shoot into sky
        let unit_direction = ray.direction.normalize();
        let t = 0.5 * (unit_direction.y + 1.0);
        return (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0);
    }
}

impl Integrator for MonteCarloPathTrace {
    fn get_radiance(&self, ray: &Ray) -> Color {
        return self.trace(ray, 50);
    }
}
