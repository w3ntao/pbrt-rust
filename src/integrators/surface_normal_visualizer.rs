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
        aggregate: Arc<dyn Primitive>,
        sampler: &mut dyn Sampler,
        camera: Arc<dyn Camera>,
        film: &mut Arc<Mutex<SimpleRGBFilm>>,
    ) {
        // TODO: rewrite sampler initialization
        // TODO: rewrite this function to evaluate a row in a time
        // to reduce concurrent access to shared data

        let filter = film.lock().unwrap().filter.clone();

        let camera_sample = sampler.get_camera_sample(p_pixel.clone(), filter);

        let camera_ray = camera.generate_camera_ray(camera_sample);

        let color = match aggregate.intersect(&camera_ray, Float::INFINITY) {
            None => RGBColor::black(),
            Some(shape_intersection) => {
                //let normal = shape_intersection.normal;
                //RGBColor::new(normal.x.abs(), normal.y.abs(), normal.z.abs())

                Vector3f::from(shape_intersection.normal)
                    .normalize()
                    .softmax_color()
            }
        };

        film.lock().unwrap().add_sample(p_pixel, color);
    }
}
