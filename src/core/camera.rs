use crate::core::pbrt::*;

pub trait Camera: Send + Sync {
    fn get_stratified_rays(
        &self,
        num_samples: u32,
        min_u: f32,
        max_u: f32,
        min_v: f32,
        max_v: f32,
    ) -> Vec<Ray>;

    fn get_ray(
        &self,
        min_u: f32,
        max_u: f32,
        min_v: f32,
        max_v: f32,
        sampler: &mut dyn Sampler,
    ) -> Ray {
        panic!("get_ray() not implemented");
    }
}
