use crate::core::pbrt::*;

pub trait Camera: Send + Sync {
    fn get_ray(
        &self,
        ndc_x: f32,
        ndc_y: f32,
        width: usize,
        height: usize,
        sampler: &mut dyn Sampler,
    ) -> Ray;

    fn reset_lens_and_focus_distance(
        &self,
        lens_radius: f32,
        focus_distance: f32,
    ) -> Arc<Perspective>;
}
