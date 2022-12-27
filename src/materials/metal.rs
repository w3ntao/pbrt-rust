use crate::core::pbrt::*;

pub struct Metal {
    albedo: Color,
    fuzz: f32,
}

impl Metal {
    pub fn new(_albedo: Color, _fuzz: f32) -> Metal {
        Metal {
            albedo: _albedo,
            fuzz: _fuzz,
        }
    }
}

fn random_in_unit_sphere(sample: Sample2D) -> Vector3 {
    let (random_u, random_v) = sample;

    let phi = random_u * 2.0 * PI;

    let sin_phi = phi.sin();
    let cos_phi = phi.cos();

    let cos_theta = random_v * 2.0 - 1.0;
    let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

    return Vector3::new(sin_phi * sin_theta, cos_phi * sin_theta, cos_theta);
}

impl Material for Metal {
    fn scatter(
        &self,
        incoming_ray: Ray,
        surface_interaction: &SurfaceInteraction,
        scattered_direction: &mut Vector3,
        attenuation: &mut Color,
        sampler: &mut dyn Sampler,
    ) -> bool {
        let reflected = incoming_ray.d.reflect(surface_interaction.n);
        let scattered_dir =
            reflected + self.fuzz * random_in_unit_sphere(sampler.get_brdf_sample());
        if surface_interaction.n.dot(scattered_dir) <= 0.0 {
            return false;
        }

        *scattered_direction = scattered_dir;
        *attenuation = self.albedo;

        return true;
    }

    fn is_specular(&self) -> bool {
        return true;
    }
}
