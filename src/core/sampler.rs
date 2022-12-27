use crate::core::pbrt::*;

pub type Sample2D = (f32, f32);

pub trait Sampler: Send + Sync {
    fn fork(&self) -> Box<dyn Sampler>;

    fn preprocess(&mut self, samples_per_pixel: usize, dimensions: usize, light_num: usize);

    fn update_round(&mut self);
    // 1 round = 1 ray

    fn get_camera_ray_sample(&mut self) -> Sample2D;

    fn get_brdf_sample(&mut self) -> Sample2D;

    fn get_light_id_sample(&mut self) -> usize;

    fn get_light_area_sample(&mut self) -> Sample2D;
}
