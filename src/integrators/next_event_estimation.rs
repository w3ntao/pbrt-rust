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
        let (light_id, light_point, light_normal, light_area) = self.world.sample_light();
        let towards_light = light_point - surface_interaction.p;
        let distance_squared = towards_light.length_squared();
        let towards_light = towards_light.normalize();

        // sampled light at the back side of object normal
        if surface_interaction.n.dot(towards_light) <= 0.0 {
            return Color::black();
        }

        let shadow_ray = Ray::new(surface_interaction.p, towards_light);

        // with light_cosine, the light emits uni-directionally
        let light_cosine = light_normal.cosine(-towards_light);
        if light_cosine <= 0.0 {
            return Color::black();
        }

        let shadow_surface_interaction =
            self.world
                .intersect(&shadow_ray, INTERSECT_OFFSET, f32::INFINITY);

        // couldn't reach the sampled light
        if !shadow_surface_interaction.intersected()
            || shadow_surface_interaction.object_id != light_id
        {
            return Color::black();
        }

        let sample_light_pdf = distance_squared / (light_cosine * light_area);

        return shadow_surface_interaction
            .material
            .emit(&shadow_surface_interaction)
            * surface_interaction.material.scattering_pdf(
                ray.d,
                surface_interaction.n,
                towards_light,
            )
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
            let surface_interaction = self.world.intersect(&ray, INTERSECT_OFFSET, f32::INFINITY);
            // with INTERSECT_OFFSET, we can avoid the situation when the ray
            // re-hit the surface it just leave

            if !surface_interaction.intersected() {
                break;
            }

            let emission = surface_interaction.material.emit(&surface_interaction);

            let (scattered, scattered_ray, attenuation) = surface_interaction
                .material
                .scatter(ray, &surface_interaction);
            if !scattered {
                if depth == 0 || last_hit_specular {
                    if surface_interaction.n.dot(ray.d) < 0.0 {
                        radiance += throughput * emission;
                    }
                }
                break;
            }

            if surface_interaction.n.dot(ray.d) < 0.0 {
                // so the light emits uni-directionally
                radiance += throughput * emission;
            }

            last_hit_specular = surface_interaction.material.is_specular();
            if !last_hit_specular {
                radiance += throughput
                    * attenuation
                    * self.get_direct_illumination(&surface_interaction, &ray);
            }

            if depth > 5 {
                let russian_roulette_probability =
                    throughput.max_val().min(RUSSIAN_ROULETTE_THRESHOLD);
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
