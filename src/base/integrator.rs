use crate::pbrt::*;

pub trait Integrator {
    fn evaluate_pixel_sample(
        &self,
        p_pixel: Point2i,
        sample_index: usize,
        sampler: &mut dyn Sampler,
        camera: Arc<Mutex<dyn Camera>>,
        world: Arc<dyn Primitive>,
    );
}
