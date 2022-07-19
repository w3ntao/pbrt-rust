use crate::fundamental::color::*;
use crate::fundamental::constants::INTERSECT_OFFSET;
use crate::fundamental::random_generator::RandomF32Generator;
use crate::ray_tracing::integrator::Integrator;
use crate::ray_tracing::ray::Ray;
use crate::ray_tracing::world::World;
use std::sync::Arc;

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
    fn trace(&self, ray: Ray) -> Color {
        let mut color = Color::black();
        let mut throughput = Color::new(1.0, 1.0, 1.0);
        let mut ray = ray;

        let mut random_generator = RandomF32Generator::new(0.0, 1.0);
        let max_russian_roulette_threshold = 0.75;

        //for depth in 0..self.max_depth {
        for depth in 0..u32::MAX {
            let intersection = self.world.intersect(&ray, INTERSECT_OFFSET, f32::INFINITY);
            // with INTERSECT_OFFSET, we can avoid the situation when the ray
            // re-hit the surface it just leave

            if !intersection.intersected() {
                color += throughput * self.background;
                break;
            }

            let emission = intersection.material.emit(&intersection);
            color += emission * throughput;

            let (scattered, scattered_ray, attenuation) =
                intersection.material.scatter(ray, &intersection);
            if !scattered {
                break;
            }

            throughput *= attenuation;

            if depth > 5 {
                let russian_roulette_prob =
                    throughput.max_val().min(max_russian_roulette_threshold);
                if random_generator.generate() > russian_roulette_prob {
                    break;
                }
                throughput /= russian_roulette_prob;
            }

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
