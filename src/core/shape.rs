use crate::core::pbrt::*;

pub trait Shape: Send + Sync {
    fn intersect(&self, ray: &Ray, t_hit: &mut f32) -> Option<SurfaceInteraction>;

    fn get_bounds(&self) -> Bounds;

    fn sample(&self, _sampler: &mut dyn Sampler) -> (Point, Vector3) {
        panic!("sample() not implemented for this Shape");
    }

    fn get_area(&self) -> f32 {
        panic!("get_area() not implemented for this Shape");
    }
}
