use crate::pbrt::*;

pub trait Integrator: Send + Sync {
    fn evaluate_pixel_sample(
        &self,
        p_pixel: Point2i,
        sampler: &mut dyn Sampler,
        camera: Arc<dyn Camera>,
        filter: Arc<dyn Filter>,
        film: &mut Arc<Mutex<dyn Film>>,
    ) {
        let lu = sampler.get_1d();
        let lambda = film.lock().unwrap().sample_wavelengths(lu);

        let camera_sample = sampler.get_camera_sample(p_pixel.clone(), filter.clone());

        let camera_ray = camera.generate_camera_ray(camera_sample, lambda);

        let L = camera_ray.weight * self.li(&camera_ray.ray, lambda, sampler);

        film.lock()
            .unwrap()
            .add_sample(p_pixel, &L, &lambda, camera_sample.filter_weight);
    }

    fn li(
        &self,
        ray: &dyn Ray,
        lambda: SampledWavelengths,
        sampler: &mut dyn Sampler,
    ) -> SampledSpectrum;

    fn fast_intersect(&self, ray: &dyn Ray, t_max: Float) -> bool {
        panic!("not implemented");
    }
}
