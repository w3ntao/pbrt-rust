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

impl Material for Metal {
    fn scatter(
        &self,
        incoming_ray: Ray,
        surface_interaction: &SurfaceInteraction,
        scattered_direction: &mut Vector3,
        attenuation: &mut Color,
    ) -> bool {
        let reflected = incoming_ray.d.reflect(surface_interaction.n);
        let scattered_dir = reflected + self.fuzz * random_in_unit_sphere();
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
