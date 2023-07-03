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
        shapes: Vec<Arc<dyn Shape>>,
    ) {
        // TODO: rewrite sampler initialization

        let film = camera.lock().unwrap().get_film();

        let filter = film.lock().unwrap().filter.clone();

        let camera_sample = sampler.get_camera_sample(p_pixel.clone(), filter);

        let camera_ray = camera.lock().unwrap().generate_camera_ray(camera_sample);

        for shape in shapes {
            match shape.intersect(&camera_ray, Float::INFINITY) {
                None => {
                    continue;
                }

                Some(shape_intersection) => {
                    let color = shape_intersection.normal.normalize().softmax_color();

                    film.lock().unwrap().add_sample(p_pixel, color);
                    return;
                }
            }
        }
        //panic!("ray missed on {}", pPixel);
        // TODO: 07/01 implementing
    }
}