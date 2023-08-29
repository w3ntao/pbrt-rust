use crate::pbrt::*;

pub trait Integrator: Send + Sync {
    fn evaluate_pixel_sample(
        &self,
        p_pixel: Point2i,
        num_samples: usize,
        aggregate: Arc<BVHAggregate>,
        sampler: &mut dyn Sampler,
        camera: Arc<dyn Camera>,
        film: &mut Arc<Mutex<dyn Film>>,
    ) {
        // TODO: rewrite sampler initialization
        // TODO: rewrite this function to evaluate a row in a time
        // to reduce concurrent access to shared data

        let filter = film.lock().unwrap().get_filter().clone();
        let mut accumulated_spectrum = RGBColor::black();
        for _ in 0..num_samples {
            let camera_sample = sampler.get_camera_sample(p_pixel.clone(), filter.clone());
            let camera_ray = camera.generate_camera_ray(camera_sample);

            accumulated_spectrum += self.Li(camera_ray, aggregate.clone());
        }

        film.lock()
            .unwrap()
            .add_sample(p_pixel, accumulated_spectrum / (num_samples as Float));
    }

    fn Li(&self, camera_ray: Ray, aggregate: Arc<BVHAggregate>) -> RGBColor;
}
