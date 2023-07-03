use crate::pbrt::*;

pub trait Integrator {
    fn evaluate_pixel_sample(
        &self,
        p_pixel: Point2i,
        camera: Arc<Mutex<dyn Camera>>,
        shapes: Vec<Arc<dyn Shape>>,
    );
}
