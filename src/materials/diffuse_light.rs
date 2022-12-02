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
        _incoming_ray: Ray,
        _surface_interaction: &SurfaceInteraction,
        _scattered_direction: &mut Vector3,
        _attenuation: &mut Color,
    ) -> bool {
        return false;
    }

    fn emit(&self, _emission: &mut Color) -> bool {
        *_emission = self.emission;
        return true;
    }
}
