use crate::pbrt::*;

pub trait Integrator: Send + Sync {
    fn li(
        &self,
        ray: &DifferentialRay,
        lambda: &SampledWavelengths,
        sampler: &mut dyn Sampler,
    ) -> SampledSpectrum;

    fn fast_intersect(&self, ray: &Ray, t_max: Float) -> bool {
        panic!("not implemented");
    }
}

pub trait IntegratorDefaultInterface: Integrator {
    fn evaluate_pixel_sample(
        &self,
        p_pixel: Point2i,
        sampler: &mut dyn Sampler,
        camera: Arc<dyn Camera>,
        filter: Arc<dyn Filter>,
        film: &mut dyn Film,
    ) {
        let lu = sampler.get_1d();
        let lambda = SampledWavelengths::sample_visible(lu);

        let camera_sample = sampler.get_camera_sample(p_pixel.clone(), filter.clone());

        let camera_ray = camera.generate_camera_differential_ray(camera_sample);

        let l = camera_ray.weight * self.li(&camera_ray.ray, &lambda, sampler);

        film.add_sample(p_pixel, &l, &lambda, camera_sample.filter_weight);
    }
}

impl<T: ?Sized + Integrator> IntegratorDefaultInterface for T {}
