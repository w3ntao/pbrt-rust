use crate::pbrt::*;

pub struct SceneConfig {
    integrator: Arc<dyn Integrator>,
    aggregate: Arc<BVHAggregate>,
    sampler: Arc<dyn Sampler>,
    camera: Arc<dyn Camera>,
    film: Arc<Mutex<SimpleRGBFilm>>,
}

impl SceneConfig {
    pub fn new(
        integrator: Arc<dyn Integrator>,
        aggregate: Arc<BVHAggregate>,
        sampler: Arc<dyn Sampler>,
        camera: Arc<dyn Camera>,
        film: Arc<Mutex<SimpleRGBFilm>>,
    ) -> Self {
        return SceneConfig {
            integrator,
            aggregate,
            sampler,
            camera,
            film,
        };
    }

    pub fn render(&mut self) {
        let resolution = self.film.lock().unwrap().resolution;
        let num_samples = 1;

        let mut forked_sampler = self.sampler.fork();
        let mutated_sampler = forked_sampler.as_mut();

        for y in 0..resolution.y {
            for x in 0..resolution.x {
                for sample_index in 0..num_samples {
                    self.integrator.evaluate_pixel_sample(
                        Point2i::new(x, y),
                        sample_index,
                        self.aggregate.clone(),
                        mutated_sampler,
                        self.camera.clone(),
                        self.film.clone(),
                    );
                }
            }
        }

        self.film.lock().unwrap().save_image();
    }
}
