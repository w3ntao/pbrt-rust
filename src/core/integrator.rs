use crate::core::pbrt::*;

pub trait Integrator: Send + Sync {
    fn get_radiance(&self, ray: Ray) -> Color;
}
