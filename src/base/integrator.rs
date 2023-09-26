use crate::pbrt::*;

pub trait Integrator: Send + Sync {
    //TODO: rewrite Integrator to copy aggregate, sampler, camera
    fn evaluate_pixel_sample(
        &self,
        p_pixel: Point2i,
        num_samples: usize,
        sampler: &mut dyn Sampler,
        camera: Arc<dyn Camera>,
        film: &mut Arc<Mutex<dyn Film>>,
    ) {
        // TODO: rewrite sampler initialization
        // TODO: rewrite this function to evaluate a row in a time
        // to reduce concurrent access to shared data

        let filter = film.lock().unwrap().get_filter().clone();
        let mut accumulated_spectrum = RGB::black();
        for _ in 0..num_samples {
            let camera_sample = sampler.get_camera_sample(p_pixel.clone(), filter.clone());
            let camera_ray = camera.generate_camera_ray(camera_sample);

            accumulated_spectrum += self.li(&camera_ray, sampler);
        }

        film.lock()
            .unwrap()
            .add_sample(p_pixel, accumulated_spectrum / (num_samples as Float));
    }

    fn li(&self, ray: &dyn Ray, sampler: &mut dyn Sampler) -> RGB;

    fn fast_intersect(&self, ray: &dyn Ray, t_max: Float) -> bool {
        panic!("not implemented");
    }
}
