use crate::pbrt::*;

pub trait Integrator {
    fn evaluate_pixel_sample(
        &self,
        p_pixel: Point2i,
        sample_index: usize,
        aggregate: Arc<dyn Primitive>,
        sampler: &mut dyn Sampler,
        camera: Arc<dyn Camera>,
        film: Arc<Mutex<SimpleRGBFilm>>,
    );
}
