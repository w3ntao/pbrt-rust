use crate::core::pbrt::*;

pub struct Mirror {}

impl Mirror {
    pub fn new() -> Mirror {
        Mirror {}
    }
}

impl Material for Mirror {
    fn scatter(
        &self,
        incoming_ray: Ray,
        surface_interaction: &SurfaceInteraction,
        scattered_direction: &mut Vector3,
        attenuation: &mut Color,
        _sampler: &mut dyn Sampler,
    ) -> bool {
        *scattered_direction = incoming_ray.d.reflect(surface_interaction.n);
        *attenuation = Color::new(1.0, 1.0, 1.0);
        return true;
    }

    fn is_specular(&self) -> bool {
        return true;
    }
}
