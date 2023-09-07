use crate::pbrt::*;

pub struct SurfaceNormalVisualizer {
    aggregate: Arc<dyn Primitive>,
}

impl SurfaceNormalVisualizer {
    pub fn new(aggregate: Arc<dyn Primitive>) -> Self {
        return SurfaceNormalVisualizer { aggregate };
    }
}

impl Integrator for SurfaceNormalVisualizer {
    fn Li(&self, camera_ray: &Ray, sampler: &mut dyn Sampler) -> RGBColor {
        return match self.aggregate.intersect(&camera_ray, Float::INFINITY) {
            None => RGBColor::black(),
            Some(shape_intersection) => {
                let n = shape_intersection.interaction.n.face_forward(-camera_ray.d);

                Vector3f::from(n).normalize().softmax_color()
            }
        };
    }
}
