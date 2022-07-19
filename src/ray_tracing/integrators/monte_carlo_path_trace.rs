use std::sync::Arc;

use crate::fundamental::color::*;
use crate::fundamental::constants::INTERSECT_OFFSET;
use crate::ray_tracing::integrator::Integrator;
use crate::ray_tracing::ray::Ray;
use crate::ray_tracing::world::World;

pub struct MonteCarloPathTrace {
    world: Arc<World>,
    background: Color,
    max_depth: u32,
}

impl MonteCarloPathTrace {
    pub fn new(_world: Arc<World>, _background: Color) -> Self {
        return Self {
            world: _world,
            background: _background,
            max_depth: 50,
        };
    }
}

impl MonteCarloPathTrace {
    fn trace(&self, ray: Ray) -> Color {
        let mut color = Color::black();
        let mut throughput = Color::new(1.0, 1.0, 1.0);

        let mut ray = ray;
        for depth in 0..self.max_depth {
            if depth == self.max_depth {
                return Color::black();
            }

            let intersection = self.world.intersect(&ray, INTERSECT_OFFSET, f32::INFINITY);
            // with INTERSECT_OFFSET, we can avoid the situation when the ray
            // re-hit the surface it just leave

            if !intersection.intersected() {
                color += throughput * self.background;
                break;
            }

            let emission = intersection.material.emit(&intersection);

            let (scattered, scattered_ray, attenuation) = intersection.material.scatter(ray, &intersection);

            if !scattered {
                color += emission * throughput;
                break;
            }

            color += emission * throughput;
            throughput *= attenuation;

            ray = scattered_ray;
        }

        return color;
    }
}

impl Integrator for MonteCarloPathTrace {
    fn get_radiance(&self, ray: Ray) -> Color {
        return self.trace(ray);
    }
}
