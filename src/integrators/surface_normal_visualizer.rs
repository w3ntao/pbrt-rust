use crate::pbrt::*;

pub struct SurfaceNormalVisualizer {}

impl SurfaceNormalVisualizer {
    pub fn new() -> Self {
        return SurfaceNormalVisualizer {};
    }
}

impl Integrator for SurfaceNormalVisualizer {
    fn evaluate_pixel_sample(
        &self,
        p_pixel: Point2i,
        sample_index: usize,
        sampler: &mut dyn Sampler,
        camera: Arc<Mutex<dyn Camera>>,
        aggregate: Arc<dyn Primitive>,
    ) {
        // TODO: rewrite sampler initialization

        let film = camera.lock().unwrap().get_film();

        let filter = film.lock().unwrap().filter.clone();

        let camera_sample = sampler.get_camera_sample(p_pixel.clone(), filter);

        let camera_ray = camera.lock().unwrap().generate_camera_ray(camera_sample);

        let color = match aggregate.intersect(&camera_ray, Float::INFINITY) {
            None => RGBColor::black(),
            Some(shape_intersection) => Vector3f::from(shape_intersection.normal)
                .normalize()
                .softmax_color(),
        };

        film.lock().unwrap().add_sample(p_pixel, color);
    }
}
