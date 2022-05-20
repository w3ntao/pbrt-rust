use crate::fundamental::color::*;
use crate::fundamental::point::Point;
use crate::ray_tracing::intersection::*;
use crate::ray_tracing::ray::*;

pub trait Material: Send + Sync {
    fn scatter(&self, incoming_ray: &Ray, intersection: &Intersection) -> (Ray, Color);
    fn emit(&self, u: f32, v: f32, point: Point) -> Color;
}

pub trait NullMaterialPredicate {
    fn is_null(&self) -> bool;
}

pub struct NullMaterial {}

impl Material for NullMaterial {
    fn scatter(&self, _: &Ray, _: &Intersection) -> (Ray, Color) {
        panic!("You should never invoke `scatter()` from NullMaterial");
    }
    fn emit(&self, _: f32, _: f32, _: Point) -> Color {
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
