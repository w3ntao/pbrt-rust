use crate::core::pbrt::*;

pub trait Shape: Send + Sync {
    fn intersect(&self, ray: &Ray, t_hit: &mut f32, interaction: &mut SurfaceInteraction) -> bool;

    fn get_bounds(&self) -> Bounds;

    fn sample(&self, sampler: &mut dyn Sampler) -> (Point, Vector3) {
        panic!("sample() not implemented for this Shape");
    }

    fn get_area(&self) -> f32 {
        panic!("get_area() not implemented for this Shape");
    }
}
