use crate::pbrt::*;

pub struct RandomWalkIntegrator {
    illuminant_spectrum: &'static dyn Spectrum,
    illuminant_scale: Float,
    base: IntegratorBase,
}

impl RandomWalkIntegrator {
    pub fn new(
        illuminant_spectrum: &'static dyn Spectrum,
        aggregate: Arc<dyn Primitive>,
        camera: Arc<dyn Camera>,
        lights: Vec<Arc<dyn Light>>,
    ) -> Self {
        let illuminant_scale = 1.0 / illuminant_spectrum.to_photometric();

        let mut infinite_lights = vec![];
        for _light in &lights {
            if _light.light_type() == LightType::Infinite {
                infinite_lights.push(_light.clone());
            }
        }

        return Self {
            base: IntegratorBase {
                aggregate,
                camera,
                lights,
                infinite_lights,
            },
            illuminant_spectrum,
            illuminant_scale,
        };
    }

    fn random_walk_li(
        &self,
        ray: &DifferentialRay,
        lambda: &SampledWavelengths,
        sampler: &mut dyn Sampler,
        depth: usize,
    ) -> SampledSpectrum {
        let mut isect = match self.base.aggregate.intersect(&ray.ray, Float::INFINITY) {
            None => {
                // Return emitted light from infinite light sources
                // to be change later to infinite lights
                return self.illuminant_scale * self.illuminant_spectrum.sample(&lambda);
            }
            Some(shape_intersection) => shape_intersection.surface_interaction,
        };

        // Terminate random walk if maximum depth has been reached
        if depth >= 5 {
            return SampledSpectrum::same_value(0.0);
        }

        // Compute BSDF at random walk intersection point
        let bsdf = isect.get_bsdf(ray, lambda, self.base.camera.as_ref(), sampler);
        if bsdf.bxdf.is_none() {
            return SampledSpectrum::same_value(0.0);
        }

        let wo = -ray.ray.d;

        // Randomly sample direction leaving surface for random walk
        let u = sampler.get_2d();
        let wp = Vector3f::sample_uniform_sphere(u);

        // Evaluate BSDF at surface for sampled direction
        let fcos =
            bsdf.f(wo, wp, TransportMode::Radiance) * wp.dot(Vector3f::from(isect.shading.n)).abs();

        if !fcos.is_positive() {
            return SampledSpectrum::same_value(0.0);
        }

        // Recursively trace ray to estimate incident radiance at surface
        let ray = isect.spawn_ray(wp);
        return fcos * (4.0 * PI) * self.random_walk_li(&ray, lambda, sampler, depth + 1);
    }
}

impl Integrator for RandomWalkIntegrator {
    fn fast_intersect(&self, ray: &Ray, t_max: Float) -> bool {
        return self.base.aggregate.fast_intersect(ray, t_max);
    }

    fn li(
        &self,
        ray: &DifferentialRay,
        lambda: &SampledWavelengths,
        sampler: &mut dyn Sampler,
    ) -> SampledSpectrum {
        return self.random_walk_li(ray, lambda, sampler, 0);
    }
}
