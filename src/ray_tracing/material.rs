use crate::fundamental::color::*;
use crate::ray_tracing::intersection::*;
use crate::ray_tracing::ray::*;

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
}

pub struct NullMaterial {}

impl Material for NullMaterial {
    fn is_null(&self) -> bool {
        return true;
    }
}

