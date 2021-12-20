use crate::fundamental::rgb_color::*;
use crate::ray_tracing::ray::*;
use crate::ray_tracing::intersection::*;
use crate::ray_tracing::material::Material;

pub struct NullMaterial {
}

impl Material for NullMaterial {
    fn scatter(&self, attenuation: &mut RGBColor, scattered_ray: &mut Ray, incoming_ray: &Ray, intersect: &Intersection) -> bool {
        panic!("You should never invoke `scatter` from Null");
    }
}
