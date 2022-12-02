use crate::core::pbrt::*;

pub trait Material: Send + Sync {
    fn scatter(
        &self,
        _incoming_ray: Ray,
        _surface_interaction: &SurfaceInteraction,
        _scattered_direction: &mut Vector3,
        _attenuation: &mut Color,
    ) -> bool {
        panic!("scatter() not implemented for this Material");
    }

    fn emit(&self, _emission: &mut Color) -> bool {
        return false;
    }

    fn is_specular(&self) -> bool {
        return false;
    }

    fn scattering_pdf(
        &self,
        _incoming_direction: Vector3,
        _normal: Normal,
        _scattered_direction: Vector3,
    ) -> f32 {
        panic!("scattering_pdf() not implemented for this Material");
    }
}
