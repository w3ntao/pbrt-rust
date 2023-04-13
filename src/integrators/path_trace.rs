use crate::core::pbrt::*;

pub struct PathTrace {
    background: Color,
}

impl Default for PathTrace {
    fn default() -> Self {
        return Self {
            background: Color::black(),
        };
    }
}

impl PathTrace {
    pub fn new(_background: Color) -> Self {
        return Self {
            background: _background,
        };
    }
}

impl Integrator for PathTrace {
    fn get_radiance(&self, ray: Ray, scene: Arc<Scene>, sampler: &mut dyn Sampler) -> Color {
        let mut radiance = Color::black();
        let mut throughput = Color::new(1.0, 1.0, 1.0);
        let mut ray = ray;

        for depth in 0..u32::MAX {
            let interaction = match scene.intersect(&ray) {
                None => {
                    radiance += throughput * self.background;
                    break;
                }
                Some(si) => si,
            };

            if interaction.n.dot(ray.d) < 0.0 {
                // so the light emits uni-directionally
                let mut emission = Color::black();

                if interaction
                    .material
                    .as_ref()
                    .expect("material is None")
                    .emit(&mut emission)
                {
                    radiance += throughput * emission;
                }
            }

            let mut scattered_direction = Vector3::invalid();
            let mut attenuation = Color::black();
            if !interaction
                .material
                .as_ref()
                .expect("material is None")
                .scatter(
                    ray,
                    &interaction,
                    &mut scattered_direction,
                    &mut attenuation,
                    sampler,
                )
            {
                break;
            }

            throughput *= attenuation;

            if depth > DEPTH_START_RUSSIAN_ROULETTE {
                let russian_roulette_probability =
                    throughput.max_component().min(RUSSIAN_ROULETTE_THRESHOLD);

                if sampler.get_1d_sample() > russian_roulette_probability {
                    break;
                }

                throughput /= russian_roulette_probability;
            }

            ray = interaction.spawn_ray(scattered_direction);
        }

        return radiance;
    }
}
