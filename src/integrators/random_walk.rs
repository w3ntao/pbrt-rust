use crate::pbrt::*;

pub struct RandomWalkIntegrator {
    illuminant_spectrum: &'static dyn Spectrum,
    illuminant_scale: Float,
    aggregate: Arc<dyn Primitive>,
}

impl RandomWalkIntegrator {
    pub fn new(illuminant_spectrum: &'static dyn Spectrum, aggregate: Arc<dyn Primitive>) -> Self {
        let illuminant_scale = 1.0 / illuminant_spectrum.to_photometric();

        return Self {
            illuminant_spectrum,
            illuminant_scale,
            aggregate,
        };
    }

    fn random_walk_li(
        &self,
        ray: &DifferentialRay,
        lambda: &SampledWavelengths,
        sampler: &mut dyn Sampler,
        depth: usize,
    ) -> SampledSpectrum {
        //TODO: implement RandomWalkIntegrator to test Material and Texture
        unreachable!();
    }
}

impl Integrator for RandomWalkIntegrator {
    fn li(
        &self,
        ray: &DifferentialRay,
        lambda: SampledWavelengths,
        sampler: &mut dyn Sampler,
    ) -> SampledSpectrum {
        //TODO: implement RandomWalkIntegrator to test Material and Texture
        unreachable!();
    }

    fn fast_intersect(&self, ray: &Ray, t_max: Float) -> bool {
        unreachable!();
    }
}
