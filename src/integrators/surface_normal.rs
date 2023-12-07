use crate::pbrt::*;

pub struct SurfaceNormal {
    aggregate: Arc<dyn Primitive>,
    rgb: [RGBAlbedoSpectrum; 3],
}

impl SurfaceNormal {
    pub fn new(aggregate: Arc<dyn Primitive>, color_space: &RGBColorSpace) -> Self {
        let val = 0.01;
        return SurfaceNormal {
            aggregate,
            rgb: color_space.generate_albedo_rgb(),
        };
    }
}

impl Integrator for SurfaceNormal {
    fn li(
        &self,
        camera_ray: &DifferentialRay,
        lambda: &SampledWavelengths,
        _sampler: &mut dyn Sampler,
    ) -> SampledSpectrum {
        return match self.aggregate.intersect(&camera_ray.ray, Float::INFINITY) {
            None => SampledSpectrum::zero(),
            Some(shape_intersection) => {
                let n = shape_intersection
                    .interaction
                    .n
                    .face_forward(-camera_ray.ray.d);

                let color = Vector3f::from(n).normalize().softmax_color();

                color.r * self.rgb[0].sample(lambda)
                    + color.g * self.rgb[1].sample(lambda)
                    + color.b * self.rgb[2].sample(lambda)
            }
        };
    }
}
