use crate::core::pbrt::*;

pub trait Integrator: Send + Sync {
    fn get_radiance(&self, ray: Ray, scene: Arc<Scene>, sampler: &mut dyn Sampler) -> Color;
}

pub const RUSSIAN_ROULETTE_THRESHOLD: f32 = 0.8;

pub const DEPTH_START_RUSSIAN_ROULETTE: u32 = 5;
