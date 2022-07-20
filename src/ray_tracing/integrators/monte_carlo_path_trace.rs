use crate::fundamental::color::*;
use crate::fundamental::constants::INTERSECT_OFFSET;
use crate::fundamental::random::RandomF32Generator;
use crate::fundamental::vector3::dot;
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

const RUSSIAN_ROULETTE_THRESHOLD: f32 = 0.8;

impl Integrator for MonteCarloPathTrace {
    fn get_radiance(&self, ray: Ray) -> Color {
        let mut radiance = Color::black();
        let mut throughput = Color::new(1.0, 1.0, 1.0);
        let mut ray = ray;

        let mut random_generator = RandomF32Generator::new(0.0, 1.0);

        for depth in 0..u32::MAX {
            let intersection = self.world.intersect(&ray, INTERSECT_OFFSET, f32::INFINITY);
            // with INTERSECT_OFFSET, we can avoid the situation when the ray
            // re-hit the surface it just leave

            if !intersection.intersected() {
                radiance += throughput * self.background;
                break;
            }

            if dot(intersection.normal, ray.direction) < 0.0 {
                // so the light emits uni-directionally
                radiance += throughput * intersection.material.emit(&intersection);
            }

            let (scattered, scattered_ray, attenuation) =
                intersection.material.scatter(ray, &intersection);
            if !scattered {
                break;
            }

            throughput *= attenuation;

            if depth > 5 {
                let russian_roulette_probability =
                    throughput.max_val().min(RUSSIAN_ROULETTE_THRESHOLD);
                if random_generator.generate() > russian_roulette_probability {
                    break;
                }
                throughput /= russian_roulette_probability;
            }

            ray = scattered_ray;
        }

        return radiance;
    }
}
