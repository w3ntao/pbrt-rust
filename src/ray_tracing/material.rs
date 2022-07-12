use crate::fundamental::color::*;
use crate::ray_tracing::intersection::*;
use crate::ray_tracing::ray::*;
use crate::fundamental::vector3::Vector3;

pub trait Material: Send + Sync {
    fn scatter(&self, _incoming_ray: Ray, _intersection: &Intersection) -> (bool, Ray, Color) {
        panic!("scatter() not implemented for this Material");
    }

    fn emit(&self, _intersection: &Intersection) -> Color {
        return Color::black();
    }

    fn is_null(&self) -> bool {
        return false;
    }

    fn is_specular(&self) -> bool { return false; }

    fn scattering_pdf(&self, _incoming_direction: Vector3, _normal: Vector3, _scattered_direction: Vector3) -> f32 {
        panic!("scattering_pdf() not implemented for this Material");
    }
}

pub struct NullMaterial {}

impl Material for NullMaterial {
    fn is_null(&self) -> bool {
        return true;
    }
}

