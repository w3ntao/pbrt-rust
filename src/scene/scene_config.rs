use crate::pbrt::*;

pub struct SceneConfig {
    integrator: Arc<SimpleIntegrator>,
    camera: Arc<Mutex<PerspectiveCamera>>,
    sampler: Arc<SimpleSampler>,
    shapes: Vec<Triangle>,
}

impl SceneConfig {
    pub fn new(
        integrator: Arc<SimpleIntegrator>,
        camera: Arc<Mutex<PerspectiveCamera>>,
        sampler: Arc<SimpleSampler>,
        shapes: Vec<Triangle>,
    ) -> Self {
        return SceneConfig {
            integrator,
            camera,
            sampler,
            shapes,
        };
    }

    pub fn render(&mut self) {
        let resolution = self.camera.lock().unwrap().film.lock().unwrap().resolution;

        for y in 0..resolution.y {
            for x in 0..resolution.x {
                let pixel = Point2i::new(x, y);

                self.integrator.evaluate_pixel_sample(
                    pixel,
                    self.camera.clone(),
                    self.shapes.clone(),
                );
            }
        }

        let film = self.camera.lock().unwrap().film.clone();
        let file_name = film.lock().unwrap().filename.clone();

        film.lock().unwrap().save_image(file_name.as_str());
        println!("image saved to `{}`", file_name);
    }
}
