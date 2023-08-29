use crate::pbrt::*;

pub struct SurfaceNormalVisualizer {}

impl SurfaceNormalVisualizer {
    pub fn new() -> Self {
        return SurfaceNormalVisualizer {};
    }
}

impl Integrator for SurfaceNormalVisualizer {
    fn Li(&self, camera_ray: Ray, aggregate: Arc<BVHAggregate>) -> RGBColor {
        return match aggregate.intersect(&camera_ray, Float::INFINITY) {
            None => RGBColor::black(),
            Some(shape_intersection) => Vector3f::from(shape_intersection.normal)
                .normalize()
                .softmax_color(),
        };
    }
}
