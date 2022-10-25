use crate::core::pbrt::*;

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
            let surface_interaction = self.world.intersect(&ray, INTERSECT_OFFSET, f32::INFINITY);
            // with INTERSECT_OFFSET, we can avoid the situation when the ray
            // re-hit the surface it just leave

            if !surface_interaction.intersected() {
                radiance += throughput * self.background;
                break;
            }

            if surface_interaction.n.dot(ray.d) < 0.0 {
                // so the light emits uni-directionally
                radiance += throughput * surface_interaction.material.emit(&surface_interaction);
            }

            let (scattered, scattered_ray, attenuation) = surface_interaction
                .material
                .scatter(ray, &surface_interaction);
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
