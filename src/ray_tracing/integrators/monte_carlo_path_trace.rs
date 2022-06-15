use std::sync::Arc;

use crate::fundamental::color::*;
use crate::fundamental::constants::INTERSECT_OFFSET;
use crate::ray_tracing::integrator::Integrator;
use crate::ray_tracing::ray::Ray;
use crate::ray_tracing::world::World;

pub struct MonteCarloPathTrace {
    world: Arc<World>,
    background: Color,
}

impl MonteCarloPathTrace {
    pub fn new(_world: Arc<World>, _background: Color) -> Self {
        return Self {
            world: _world,
            background: _background,
        };
    }
}

impl MonteCarloPathTrace {
    fn trace(&self, ray: Ray, depth: u32) -> Color {
        let mut ray = ray;
        let mut accumulated_attenuation = Color::new(1.0, 1.0, 1.0);

        for _ in 0..depth {
            let intersection = self.world.scene.intersect(&ray, INTERSECT_OFFSET, f32::INFINITY);
            // with INTERSECT_OFFSET, we can avoid the situation when the ray
            // re-hit the surface it just leave

            if !intersection.intersected() {
                return accumulated_attenuation * self.background;
            }

            let emission = intersection.material.emit(intersection.u, intersection.v, intersection.hit_point);

            let (scattered, scattered_ray, attenuation) = intersection.material.scatter(ray, &intersection);

            if !scattered {
                return accumulated_attenuation * emission;
            }

            ray = scattered_ray;
            accumulated_attenuation = (emission + accumulated_attenuation) * attenuation;
        }
        return Color::black();
    }
}

impl Integrator for MonteCarloPathTrace {
    fn get_radiance(&self, ray: Ray) -> Color {
        return self.trace(ray, 50);
    }
}
