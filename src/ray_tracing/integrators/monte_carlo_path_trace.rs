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
        if depth <= 0 {
            return Color::black();
        }

        let intersection = self.world.scene.intersect(&ray, INTERSECT_OFFSET, f32::INFINITY);
        // with INTERSECT_OFFSET, we can avoid the situation when the ray
        // re-hit the surface it just leave

        if !intersection.intersected() {
            return self.background;
        }

        let emission = intersection.material.emit(intersection.u, intersection.v, intersection.hit_point);

        if emission.r > 0.0 || emission.g > 0.0 || emission.b > 0.0 {
            return emission;
        }

        let (scattered_ray, attenuation) = intersection.material.scatter(ray, &intersection);

        return attenuation * self.trace(scattered_ray, depth - 1);
    }
}

impl Integrator for MonteCarloPathTrace {
    fn get_radiance(&self, ray: Ray) -> Color {
        return self.trace(ray, 50);
    }
}
