use crate::fundamental::color::*;
use crate::ray_tracing::intersection::*;
use crate::ray_tracing::ray::*;

pub trait Material: Send + Sync {
    fn scatter(&self, incoming_ray: &Ray, intersection: &Intersection, scattered_ray: &mut Ray) -> Color;
}
