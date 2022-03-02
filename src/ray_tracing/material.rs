use crate::fundamental::color::*;
use crate::ray_tracing::intersection::*;
use crate::ray_tracing::ray::*;

pub trait Material: Send + Sync {
    fn scatter(&self, scattered_ray: &mut Ray, incoming_ray: &Ray, intersection: &Intersection) -> Color;
}
