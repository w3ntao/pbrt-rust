use crate::pbrt::*;

pub struct SurfaceNormal {
    rgb: [RGBAlbedoSpectrum; 3],
    base: IntegratorBase,
}

impl SurfaceNormal {
    pub fn new(
        aggregate: Arc<dyn Primitive>,
        camera: Arc<dyn Camera>,
        color_space: &RGBColorSpace,
    ) -> Self {
        return SurfaceNormal {
            base: IntegratorBase::new(aggregate, camera, vec![]),
            rgb: color_space.generate_albedo_rgb(),
        };
    }
}

impl Integrator for SurfaceNormal {
    fn fast_intersect(&self, ray: &Ray, t_max: f64) -> bool {
        return self.base.aggregate.fast_intersect(ray, t_max);
    }

    fn li(
        &self,
        camera_ray: &DifferentialRay,
        lambda: &SampledWavelengths,
        _sampler: &mut dyn Sampler,
    ) -> SampledSpectrum {
        return match self
            .base
            .aggregate
            .intersect(&camera_ray.ray, f64::INFINITY)
        {
            None => SampledSpectrum::same_value(0.0),
            Some(shape_intersection) => {
                let n = shape_intersection
                    .surface_interaction
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
