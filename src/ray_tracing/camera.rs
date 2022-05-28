use crate::ray_tracing::ray::Ray;

pub trait Camera: Send + Sync {
    fn get_primary_ray(&self, x: f32, y: f32) -> Ray;
    fn get_stratified_rays(&self, num_samples: u32, min_u: f32, max_u: f32, min_v: f32, max_v: f32) -> Vec<Ray>;
}
