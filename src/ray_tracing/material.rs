use crate::fundamental::color::*;
use crate::ray_tracing::intersection::*;
use crate::ray_tracing::ray::*;

pub trait Material: Send + Sync {
    fn scatter(&self, incoming_ray: Ray, intersection: &Intersection) -> (bool, Ray, Color);

    fn emit(&self, _: &Intersection) -> Color {
        return Color::black();
    }
}

pub trait NullMaterialPredicate {
    fn is_null(&self) -> bool;
}

pub struct NullMaterial {}

impl Material for NullMaterial {
    fn scatter(&self, _: Ray, _: &Intersection) -> (bool, Ray, Color) {
        panic!("You should never invoke `scatter()` from NullMaterial");
    }
}

impl NullMaterialPredicate for NullMaterial {
    fn is_null(&self) -> bool {
        return true;
    }
}

impl NullMaterialPredicate for dyn Material {
    fn is_null(&self) -> bool {
        return false;
    }
}

