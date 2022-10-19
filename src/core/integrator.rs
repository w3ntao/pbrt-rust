use crate::core::ray::Ray;
use crate::fundamental::color::Color;

pub trait Integrator: Send + Sync {
    fn get_radiance(&self, ray: Ray) -> Color;
}
