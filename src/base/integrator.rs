use crate::pbrt::*;

pub trait Integrator: Send + Sync {
    fn evaluate_pixel_sample(
        &self,
        p_pixel: Point2i,
        sample_index: usize,
        aggregate: Arc<dyn Primitive>,
        sampler: &mut dyn Sampler,
        camera: Arc<dyn Camera>,
        film: &mut Arc<Mutex<dyn Film>>,
    );
}
