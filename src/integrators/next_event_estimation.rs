use crate::core::pbrt::*;

pub struct NextEventEstimation {
    world: Arc<World>,
}

impl NextEventEstimation {
    pub fn new(_world: Arc<World>) -> Self {
        return Self { world: _world };
    }
}

const RUSSIAN_ROULETTE_THRESHOLD: f32 = 0.8;

impl NextEventEstimation {
    fn get_direct_illumination(
        &self,
        surface_interaction: &SurfaceInteraction,
        ray: &Ray,
    ) -> Color {
        let (light_point, light_normal, light_area, light_material) = self.world.sample_light();
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
        let mut light_surface_interaction = SurfaceInteraction::default();
        // couldn't reach the sampled light
        if self
            .world
            .intersect(&shadow_ray, &mut light_surface_interaction)
        {
            return Color::black();
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
    fn get_radiance(&self, ray: Ray) -> Color {
        let mut radiance = Color::black();
        let mut throughput = Color::new(1.0, 1.0, 1.0);
        let mut ray = ray;
        let mut last_hit_specular = false;

        let mut random_generator = RandomF32Generator::new(0.0, 1.0);

        for depth in 0..u32::MAX {
            let mut interaction = SurfaceInteraction::default();
            // with INTERSECT_OFFSET, we can avoid the situation when the ray
            // re-hit the surface it just leave

            if !self.world.intersect(&ray, &mut interaction) {
                break;
            }

            let mut emission = Color::black();
            let emit = interaction
                .material
                .as_ref()
                .expect("material is None")
                .emit(&mut emission);

            let (scattered, scattered_ray, attenuation) = interaction
                .material
                .as_ref()
                .expect("material is None")
                .scatter(ray, &interaction);
            if !scattered {
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
                radiance +=
                    throughput * attenuation * self.get_direct_illumination(&interaction, &ray);
            }

            if depth > 5 {
                let russian_roulette_probability =
                    throughput.max_component().min(RUSSIAN_ROULETTE_THRESHOLD);
                if random_generator.generate() > russian_roulette_probability {
                    break;
                }
                throughput /= russian_roulette_probability;
            }

            throughput *= attenuation;
            ray = scattered_ray;
        }

        return radiance;
    }
}
