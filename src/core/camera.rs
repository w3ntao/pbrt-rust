use crate::core::pbrt::*;

pub trait Camera: Send + Sync {
    fn get_ray(
        &self,
        min_u: f32,
        max_u: f32,
        min_v: f32,
        max_v: f32,
        sampler: &mut dyn Sampler,
    ) -> Ray;

    fn remove_lens(&self) -> Arc<Perspective>;
}
