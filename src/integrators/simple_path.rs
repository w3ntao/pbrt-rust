use crate::pbrt::*;

pub struct SimplePath {
    max_depth: usize,
    base: IntegratorBase,
    light_sampler: UniformLightSampler,
}

impl Integrator for SimplePath {
    fn fast_intersect(&self, ray: &Ray, t_max: f64) -> bool {
        return self.base.aggregate.fast_intersect(ray, t_max);
    }

    fn li(
        &self,
        ray: &DifferentialRay,
        lambda: &SampledWavelengths,
        sampler: &mut dyn Sampler,
    ) -> SampledSpectrum {
        // Estimate radiance along ray using simple path tracing
        let mut L = SampledSpectrum::same_value(0.0);
        let mut beta = SampledSpectrum::same_value(1.0);
        let mut specular_bounce = true;
        let mut depth = 0;

        let mut ray = ray.clone();

        while beta.is_positive() {
            // Find next _SimplePathIntegrator_ vertex and accumulate contribution
            // Intersect _ray_ with scene
            let mut si = match self.base.aggregate.intersect(&ray.ray, f64::INFINITY) {
                None => {
                    if specular_bounce {
                        for light in &self.base.infinite_lights {
                            L += beta * light.le(&ray.ray, lambda);
                        }
                    }
                    break;
                }
                Some(_si) => _si,
            };

            // Account for emissive surface if light was not sampled
            let mut isect = &mut si.surface_interaction;
            if specular_bounce {
                L += beta * isect.le(-ray.ray.d, lambda);
            }

            // End path if maximum depth reached
            depth += 1;
            if depth == self.max_depth {
                break;
            }

            // Get BSDF and skip over medium boundaries
            let bsdf = isect.get_bsdf(&ray, lambda, self.base.camera.as_ref(), sampler);
            if bsdf.bxdf.is_none() {
                panic!("SimplePathIntegrator: `bsdf.bxdf.is_none()` is not implemented");
            }

            // Sample direct illumination if _sampleLights_ is true
            let wo = -ray.ray.d;
            match self.light_sampler.sample(sampler.get_1d()) {
                None => {}
                Some(sampled_light) => {
                    // Sample point on _sampledLight_ to estimate direct illumination
                    let u_light = sampler.get_2d();
                    match sampled_light.light.sample_li(
                        &LightSampleContext::from_surface_interaction(isect),
                        u_light,
                        lambda,
                        false,
                    ) {
                        None => {}

                        Some(ls) => {
                            if ls.l.is_positive() && ls.pdf > 0.0 {
                                // Evaluate BSDF for light and possibly add scattered radiance
                                let wi = ls.wi;
                                let f = bsdf.f(wo, wi, TransportMode::Radiance)
                                    * wi.dot(Vector3f::from(isect.shading.n)).abs();

                                if f.is_positive()
                                    && self.unoccluded(&isect.interaction, &ls.p_light)
                                {
                                    L += beta * f * ls.l / (sampled_light.p * ls.pdf);
                                }
                            }
                        }
                    };
                }
            };

            // Sample outgoing direction at intersection to continue path
            // Sample BSDF for new path direction
            let u = sampler.get_1d();
            let bs = match bsdf.sample_f(
                wo,
                u,
                sampler.get_2d(),
                TransportMode::Radiance,
                BxDFReflTransFlags::All,
            ) {
                None => {
                    break;
                }
                Some(_bs) => _bs,
            };

            beta *= bs.f * bs.wi.dot(Vector3f::from(isect.shading.n)).abs() / bs.pdf;
            specular_bounce = bs.is_specular();
            ray = isect.spawn_ray(bs.wi);
        }

        return L;
    }
}

impl SimplePath {
    pub fn new(base: IntegratorBase) -> Self {
        unreachable!();

        /*
        return Self {
            base,
            sample_bsdf: true,
            sample_light: true,
            max_depth: 5,
        };
        */
    }
}
