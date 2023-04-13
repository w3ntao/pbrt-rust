use crate::core::pbrt::*;

pub struct NextEventEstimation {}

impl Default for NextEventEstimation {
    fn default() -> Self {
        return Self {};
    }
}

impl NextEventEstimation {
    fn get_direct_illumination(
        &self,
        surface_interaction: &SurfaceInteraction,
        ray: &Ray,
        scene: Arc<Scene>,
        sampler: &mut dyn Sampler,
    ) -> Color {
        let (light_point, light_normal, light_area, light_material) = scene.sample_light(sampler);
        let towards_light = light_point - surface_interaction.p;
        let distance = towards_light.length();
        let towards_light = towards_light.normalize();

        // sampled light at the back side of object normal
        if surface_interaction.n.dot(towards_light) <= 0.0 {
            return Color::black();
        }

        // with light_cosine, the light emits uni-directionally
        let light_cosine = light_normal.cosine(-towards_light);
        if light_cosine <= 0.0 {
            return Color::black();
        }

        let shadow_ray = surface_interaction.spawn_shadow_ray(light_point);
        match scene.intersect(&shadow_ray) {
            None => {}
            Some(_) => {
                return Color::black();
            }
        }

        let mut emission = Color::black();
        if !light_material.emit(&mut emission) {
            return emission;
        }

        let sample_light_pdf = distance * distance / (light_cosine * light_area);
        return emission
            * surface_interaction
                .material
                .as_ref()
                .expect("material is None")
                .scattering_pdf(ray.d, surface_interaction.n, towards_light)
            / sample_light_pdf;
    }
}

impl Integrator for NextEventEstimation {
    fn get_radiance(&self, ray: Ray, scene: Arc<Scene>, sampler: &mut dyn Sampler) -> Color {
        let mut radiance = Color::black();
        let mut throughput = Color::new(1.0, 1.0, 1.0);
        let mut ray = ray;
        let mut last_hit_specular = false;

        for depth in 0..u32::MAX {
            let interaction = match scene.intersect(&ray) {
                None => {
                    break;
                }
                Some(si) => si,
            };

            let mut emission = Color::black();
            let emit = interaction
                .material
                .as_ref()
                .expect("material is None")
                .emit(&mut emission);

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
                if (depth == 0 || last_hit_specular) && emit && interaction.n.dot(ray.d) < 0.0 {
                    radiance += throughput * emission;
                }
                break;
            }

            if emit && interaction.n.dot(ray.d) < 0.0 {
                // so the light emits uni-directionally
                radiance += throughput * emission;
            }

            last_hit_specular = interaction
                .material
                .as_ref()
                .expect("material is None")
                .is_specular();
            if !last_hit_specular {
                radiance += throughput
                    * attenuation
                    * self.get_direct_illumination(&interaction, &ray, scene.clone(), sampler);
            }

            if depth > DEPTH_START_RUSSIAN_ROULETTE {
                let russian_roulette_probability =
                    throughput.max_component().min(RUSSIAN_ROULETTE_THRESHOLD);
                if sampler.get_1d_sample() > russian_roulette_probability {
                    break;
                }
                throughput /= russian_roulette_probability;
            }

            throughput *= attenuation;

            ray = interaction.spawn_ray(scattered_direction);
        }

        return radiance;
    }
}
