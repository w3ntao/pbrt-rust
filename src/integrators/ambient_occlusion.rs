use crate::pbrt::*;

pub struct AmbientOcclusion {
    illuminant_spectrum: &'static dyn Spectrum,
    illuminant_scale: Float,
    aggregate: Arc<dyn Primitive>,
}

impl AmbientOcclusion {
    pub fn new(illuminant_spectrum: &'static dyn Spectrum, aggregate: Arc<dyn Primitive>) -> Self {
        let illuminant_scale = 1.0 / illuminant_spectrum.to_photometric();

        return Self {
            illuminant_spectrum,
            illuminant_scale,
            aggregate,
        };
    }
}

impl Integrator for AmbientOcclusion {
    fn li(
        &self,
        ray: &DifferentialRay,
        lambda: &SampledWavelengths,
        sampler: &mut dyn Sampler,
    ) -> SampledSpectrum {
        // TODO: this is incomplete, consider BSDF only for now
        let si = match self.aggregate.intersect(&ray.ray, Float::INFINITY) {
            None => {
                return SampledSpectrum::zero();
            }
            Some(_si) => _si,
        };

        let isect = si.interaction;

        let n = isect.n.face_forward(-ray.ray.d);
        let u = sampler.get_2d();

        let local_wi = sample_cosine_hemisphere(u);
        let pdf = cosine_hemisphere_pdf(local_wi.z.abs());

        if pdf == 0.0 {
            return SampledSpectrum::zero();
        }

        let frame = Frame::from_z(Vector3f::from(n));
        let wi = frame.from_local(local_wi);

        // Divide by pi so that fully visible is one.
        let differential_ray = isect.spawn_ray(wi);
        if !self.fast_intersect(&differential_ray.ray, Float::INFINITY) {
            return self.illuminant_spectrum.sample(&lambda)
                * (self.illuminant_scale * n.dot(wi) / (PI * pdf));
        }

        return SampledSpectrum::zero();
    }

    fn fast_intersect(&self, ray: &Ray, t_max: Float) -> bool {
        return self.aggregate.fast_intersect(ray, t_max);
    }
}
