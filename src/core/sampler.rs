use crate::core::pbrt::*;

pub type Sample2D = (f32, f32);

pub trait Sampler: Send + Sync {
    fn fork(&self) -> Box<dyn Sampler>;

    fn preprocess(&mut self, samples_per_pixel: usize, dimensions: usize);

    fn update_round(&mut self);
    // 1 round = 1 ray

    fn get_camera_ray_sample(&mut self) -> (f32, f32);

    fn get_brdf_sample(&mut self) -> (f32, f32);
}
