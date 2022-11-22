use crate::core::pbrt::*;

pub struct DiffuseLight {
    emission: Color,
}

impl DiffuseLight {
    pub fn new(_emission: Color) -> DiffuseLight {
        DiffuseLight {
            emission: _emission,
        }
    }
}

impl Material for DiffuseLight {
    fn scatter(
        &self,
        incoming_ray: Ray,
        surface_interaction: &SurfaceInteraction,
        scattered_direction: &mut Vector3,
        attenuation: &mut Color,
    ) -> bool {
        return false;
    }

    fn emit(&self, _emission: &mut Color) -> bool {
        *_emission = self.emission;
        return true;
    }
}
