use crate::pbrt::*;

pub struct SurfaceNormal {
    aggregate: Arc<dyn Primitive>,
}

impl SurfaceNormal {
    pub fn new(aggregate: Arc<dyn Primitive>) -> Self {
        return SurfaceNormal { aggregate };
    }
}

impl Integrator for SurfaceNormal {
    fn li(&self, camera_ray: &dyn Ray, _sampler: &mut dyn Sampler) -> RGBColor {
        return match self.aggregate.intersect(camera_ray, Float::INFINITY) {
            None => RGBColor::black(),
            Some(shape_intersection) => {
                let n = shape_intersection
                    .interaction
                    .n
                    .face_forward(-camera_ray.get_d());

                Vector3f::from(n).normalize().softmax_color()
            }
        };
    }
}
