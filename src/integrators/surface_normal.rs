use crate::pbrt::*;

pub struct SurfaceNormal {
    aggregate: Arc<dyn Primitive>,
    red: RGBAlbedoSpectrum,
    green: RGBAlbedoSpectrum,
    blue: RGBAlbedoSpectrum,
}

impl SurfaceNormal {
    pub fn new(aggregate: Arc<dyn Primitive>, color_space: &RGBColorSpace) -> Self {
        let val = 0.01;
        return SurfaceNormal {
            aggregate,
            red: RGBAlbedoSpectrum::new(RGB::new(val, 0.0, 0.0), color_space),
            green: RGBAlbedoSpectrum::new(RGB::new(0.0, val, 0.0), color_space),
            blue: RGBAlbedoSpectrum::new(RGB::new(0.0, 0.0, val), color_space),
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

                color.r * self.red.sample(lambda)
                    + color.g * self.green.sample(lambda)
                    + color.b * self.blue.sample(lambda)
            }
        };
    }
}
