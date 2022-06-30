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
    fn trace(&self, ray: Ray, depth: u32, max_depth: u32) -> Color {
        if depth == max_depth {
            return Color::black();
        }

        let intersection = self.world.intersect(&ray, INTERSECT_OFFSET, f32::INFINITY);
        // with INTERSECT_OFFSET, we can avoid the situation when the ray
        // re-hit the surface it just leave

        if !intersection.intersected() {
            return self.background;
        }

        let emission = intersection.material.emit(&intersection);

        let (scattered, scattered_ray, attenuation) = intersection.material.scatter(ray, &intersection);

        if !scattered {
            return emission;
        }

        return emission + attenuation * self.trace(scattered_ray, depth + 1, max_depth);
    }
}

impl Integrator for MonteCarloPathTrace {
    fn get_radiance(&self, ray: Ray) -> Color {
        return self.trace(ray, 0, 50);
    }
}
