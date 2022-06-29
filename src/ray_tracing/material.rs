use crate::fundamental::color::*;
use crate::fundamental::point::Point;
use crate::ray_tracing::intersection::*;
use crate::ray_tracing::ray::*;

pub trait Material: Send + Sync {
    fn scatter(&self, incoming_ray: Ray, intersection: &Intersection) -> (bool, Ray, Color);
    fn emit(&self, intersection: &Intersection) -> Color;
}

pub trait NullMaterialPredicate {
    fn is_null(&self) -> bool;
}

pub trait Illumination {
    fn illuminate(&self, intersection: &Intersection) -> Color;
}

pub struct NullMaterial {}

impl Material for NullMaterial {
    fn scatter(&self, _: Ray, _: &Intersection) -> (bool, Ray, Color) {
        panic!("You should never invoke `scatter()` from NullMaterial");
    }

    fn emit(&self, _: &Intersection) -> Color {
        panic!("You should never invoke `emit()` from NullMaterial");
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

impl Illumination for dyn Material {
    fn illuminate(&self, _: &Intersection) -> Color {
        return Color::black();
    }
}
