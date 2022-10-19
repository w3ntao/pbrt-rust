use crate::core::color::Color;
use crate::core::ray::Ray;

pub trait Integrator: Send + Sync {
    fn get_radiance(&self, ray: Ray) -> Color;
}
