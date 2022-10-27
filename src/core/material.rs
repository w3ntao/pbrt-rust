use crate::core::pbrt::*;

pub trait Material: Send + Sync {
    fn scatter(
        &self,
        _incoming_ray: Ray,
        _surface_interaction: &SurfaceInteraction,
    ) -> (bool, Ray, Color) {
        panic!("scatter() not implemented for this Material");
    }

    fn emit(&self, _emission: &mut Color, _interaction: &SurfaceInteraction) -> bool {
        return false;
    }

    fn is_null(&self) -> bool {
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

pub struct NullMaterial {}

impl Material for NullMaterial {
    fn is_null(&self) -> bool {
        return true;
    }
}
