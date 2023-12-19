use crate::pbrt::*;

pub struct AmbientOcclusion {
    illuminant_spectrum: &'static dyn Spectrum,
    illuminant_scale: f64,
    base: IntegratorBase,
}

impl AmbientOcclusion {
    pub fn new(
        illuminant_spectrum: &'static dyn Spectrum,
        aggregate: Arc<dyn Primitive>,
        camera: Arc<dyn Camera>,
    ) -> Self {
        let illuminant_scale = 1.0 / illuminant_spectrum.to_photometric();

        return Self {
            base: IntegratorBase {
                aggregate,
                camera,
                lights: vec![],
                infinite_lights: vec![],
            },
            illuminant_spectrum,
            illuminant_scale,
        };
    }
}

impl Integrator for AmbientOcclusion {
    fn fast_intersect(&self, ray: &Ray, t_max: f64) -> bool {
        return self.base.aggregate.fast_intersect(ray, t_max);
    }

    fn li(
        &self,
        ray: &DifferentialRay,
        lambda: &SampledWavelengths,
        sampler: &mut dyn Sampler,
    ) -> SampledSpectrum {
        // TODO: this is incomplete, consider BSDF only for now
        let si = match self.base.aggregate.intersect(&ray.ray, f64::INFINITY) {
            None => {
                return SampledSpectrum::same_value(0.0);
            }
            Some(_si) => _si,
        };

        let isect = si.surface_interaction;

        let n = isect.interaction.n.face_forward(-ray.ray.d);
        let u = sampler.get_2d();

        let local_wi = sample_cosine_hemisphere(u);
        let pdf = cosine_hemisphere_pdf(local_wi.z.abs());

        if pdf == 0.0 {
            return SampledSpectrum::same_value(0.0);
        }

        let frame = Frame::from_z(Vector3f::from(n));
        let wi = frame.from_local(local_wi);

        // Divide by pi so that fully visible is one.
        let differential_ray = isect.spawn_ray(wi);
        if !self
            .base
            .aggregate
            .fast_intersect(&differential_ray.ray, f64::INFINITY)
        {
            return self.illuminant_spectrum.sample(&lambda)
                * (self.illuminant_scale * n.dot(wi) / (PI * pdf));
        }

        return SampledSpectrum::same_value(0.0);
    }
}
