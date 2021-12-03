use crate::fundamental::vector::Vector;
use crate::ray_tracing::ray::Ray;

pub trait Integrator {
    fn get_radiance(&self, ray: &Ray) -> Vector;
}
