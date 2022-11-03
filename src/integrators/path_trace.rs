use crate::core::pbrt::*;

pub struct PathTrace {
    world: Arc<World>,
    background: Color,
}

impl PathTrace {
    pub fn new(_world: Arc<World>, _background: Color) -> Self {
        return Self {
            world: _world,
            background: _background,
        };
    }
}

const RUSSIAN_ROULETTE_THRESHOLD: f32 = 0.8;

impl Integrator for PathTrace {
    fn get_radiance(&self, ray: Ray) -> Color {
        let mut radiance = Color::black();
        let mut throughput = Color::new(1.0, 1.0, 1.0);
        let mut ray = ray;

        let mut random_generator = RandomF32Generator::new(0.0, 1.0);

        for depth in 0..u32::MAX {
            let mut interaction = SurfaceInteraction::failure();
            if !self.world.intersect(&ray, &mut interaction) {
                radiance += throughput * self.background;
                break;
            }

            if interaction.n.dot(ray.d) < 0.0 {
                // so the light emits uni-directionally
                let mut emission = Color::black();

                if interaction
                    .material
                    .as_ref()
                    .expect("material is None")
                    .emit(&mut emission, &interaction)
                {
                    radiance += throughput * emission;
                }
            }

            let (scattered, scattered_ray, attenuation) = interaction
                .material
                .as_ref()
                .expect("material is None")
                .scatter(ray, &interaction);

            if !scattered {
                break;
            }

            throughput *= attenuation;

            if depth > 5 {
                let russian_roulette_probability =
                    throughput.max_component().min(RUSSIAN_ROULETTE_THRESHOLD);
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
