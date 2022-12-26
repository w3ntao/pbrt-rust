use crate::core::pbrt::*;

pub trait Sampler: Send + Sync {
    fn fork(&self) -> Box<dyn Sampler>;

    fn prepare(&mut self, samples_per_pixel: usize, dimensions: usize);
    // 1 round = 1 pixel

    fn update_round(&mut self);

    fn get_camera_ray_sample(&mut self) -> (f32, f32);

    fn get_brdf_sample(&mut self) -> (f32, f32);
}
