use crate::pbrt::*;

pub struct SimpleIntegrator {}

impl SimpleIntegrator {
    pub fn new() -> Self {
        return SimpleIntegrator {};
    }

    pub fn evaluate_pixel_sample(
        pPixel: Point2i,
        camera: Arc<Mutex<PerspectiveCamera>>,
        sampler: &mut SimpleSampler,
        filter: Arc<BoxFilter>,
        shapes: Vec<Triangle>,
    ) {
        let camera_sample = get_camera_sample(sampler, pPixel, filter);

        // TODO: 06/26 implementing
    }
}
