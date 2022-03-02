use crate::fundamental::color::Color;
use crate::ray_tracing::ray::Ray;

pub trait Integrator: Send + Sync {
    fn get_radiance(&self, ray: &Ray) -> Color;
}
