use crate::fundamental::rgb_color::*;
use crate::ray_tracing::ray::*;
use crate::ray_tracing::intersection::*;

pub trait Material: Send + Sync {
    fn scatter(&self, attenuation: &mut RGBColor, scattered_ray: &mut Ray, incoming_ray: &Ray, intersect: &Intersection) -> bool;
}
