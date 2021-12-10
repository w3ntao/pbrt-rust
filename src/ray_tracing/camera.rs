use crate::ray_tracing::ray::Ray;

pub trait Camera: Send + Sync {
    fn get_primary_ray(&self, x: f32, y: f32) -> Ray;
}
