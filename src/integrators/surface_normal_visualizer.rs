use crate::pbrt::*;

pub struct SurfaceNormalVisualizer {}

impl SurfaceNormalVisualizer {
    pub fn new() -> Self {
        return SurfaceNormalVisualizer {};
    }
}

impl Integrator for SurfaceNormalVisualizer {
    fn Li(
        &self,
        camera_ray: Ray,
        aggregate: Arc<BVHAggregate>,
        sampler: &mut dyn Sampler,
    ) -> RGBColor {
        return match aggregate.intersect(&camera_ray, Float::INFINITY) {
            None => RGBColor::black(),
            Some(shape_intersection) => {
                let n = shape_intersection.interaction.n.face_forward(-camera_ray.d);

                Vector3f::from(n).normalize().softmax_color()
            }
        };
    }
}
