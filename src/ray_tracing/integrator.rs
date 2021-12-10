use crate::fundamental::rgb_color::RGBColor;
use crate::ray_tracing::ray::Ray;

pub trait Integrator {
    fn get_radiance(&self, ray: &Ray) -> RGBColor;
}
