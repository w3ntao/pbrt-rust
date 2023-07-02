use crate::pbrt::*;

pub struct SceneConfig {
    integrator: Arc<SimpleIntegrator>,
    camera: Arc<Mutex<PerspectiveCamera>>,
    sampler: Arc<SimpleSampler>,
    filter: Arc<BoxFilter>,
    shapes: Vec<Triangle>,
}

impl SceneConfig {
    pub fn new(
        integrator: Arc<SimpleIntegrator>,
        camera: Arc<Mutex<PerspectiveCamera>>,
        sampler: Arc<SimpleSampler>,
        filter: Arc<BoxFilter>,
        shapes: Vec<Triangle>,
    ) -> Self {
        return SceneConfig {
            integrator,
            camera,
            sampler,
            filter,
            shapes,
        };
    }

    pub fn render(&mut self) {
        let resolution = self.camera.lock().unwrap().film.lock().unwrap().resolution;

        let sampler = SimpleSampler {
            rng: StdRng::from_entropy(),
        };

        for y in 0..resolution.y {
            for x in 0..resolution.x {
                let pixel = Point2i::new(x, y);

                self.integrator.evaluate_pixel_sample(
                    pixel,
                    self.camera.clone(),
                    self.filter.clone(),
                    self.shapes.clone(),
                );
            }
        }

        //let pixel_bounds = Bounds
    }
}
