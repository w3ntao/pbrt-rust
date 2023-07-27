use crate::accelerator::bvh::BVHAggregate;
use crate::pbrt::*;

pub struct SceneConfig {
    integrator: Arc<dyn Integrator>,
    camera: Arc<Mutex<dyn Camera>>,
    sampler: Arc<dyn Sampler>,
    aggregate: Arc<BVHAggregate>,
}

impl SceneConfig {
    pub fn new(
        integrator: Arc<dyn Integrator>,
        camera: Arc<Mutex<dyn Camera>>,
        sampler: Arc<dyn Sampler>,
        aggregate: Arc<BVHAggregate>,
    ) -> Self {
        return SceneConfig {
            integrator,
            camera,
            sampler,
            aggregate,
        };
    }

    pub fn render(&mut self) {
        let resolution = self
            .camera
            .lock()
            .unwrap()
            .get_film()
            .lock()
            .unwrap()
            .resolution;

        let num_samples = 1;

        let mut forked_sampler = self.sampler.fork();
        let mutated_sampler = forked_sampler.as_mut();

        for y in 0..resolution.y {
            for x in 0..resolution.x {
                let pixel = Point2i::new(x, y);

                for sample_index in 0..num_samples {
                    self.integrator.evaluate_pixel_sample(
                        pixel,
                        sample_index,
                        mutated_sampler,
                        self.camera.clone(),
                        self.aggregate.clone(),
                    );
                }
            }
        }

        let film = self.camera.lock().unwrap().get_film();
        let file_name = film.lock().unwrap().filename.clone();

        film.lock().unwrap().save_image(file_name.as_str());
        println!("image saved to `{}`", file_name);
    }
}
