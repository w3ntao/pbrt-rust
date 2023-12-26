use crate::pbrt::*;

pub struct RandomWalkIntegrator {
    illuminant_spectrum: &'static dyn Spectrum,
    illuminant_scale: f64,
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
            base: IntegratorBase::new(aggregate, camera, lights),
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
        let mut isect = match self.base.aggregate.intersect(&ray.ray, f64::INFINITY) {
            None => {
                // Return emitted light from infinite light sources
                // to be change later to infinite lights
                return SampledSpectrum::same_value(0.0);
            }
            Some(shape_intersection) => shape_intersection.surface_interaction,
        };

        // Get emitted radiance at surface intersection
        let wo = -ray.ray.d;
        let le = isect.le(wo, lambda);

        // Terminate random walk if maximum depth has been reached
        if depth >= 5 {
            return le;
        }

        // Compute BSDF at random walk intersection point
        let bsdf = isect.get_bsdf(ray, lambda, self.base.camera.as_ref(), sampler);
        if bsdf.bxdf.is_none() {
            return le;
        }

        // Randomly sample direction leaving surface for random walk
        let u = sampler.get_2d();
        let wp = Vector3f::sample_uniform_sphere(u);

        // Evaluate BSDF at surface for sampled direction
        let fcos = bsdf.f(wo, wp, TransportMode::Radiance) * isect.shading.n.dot(wp).abs();

        if !fcos.is_positive() {
            return le;
        }

        // Recursively trace ray to estimate incident radiance at surface
        let ray = isect.spawn_ray(wp);
        return le + fcos * self.random_walk_li(&ray, lambda, sampler, depth + 1) * (4.0 * PI);
    }
}

impl Integrator for RandomWalkIntegrator {
    fn fast_intersect(&self, ray: &Ray, t_max: f64) -> bool {
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
