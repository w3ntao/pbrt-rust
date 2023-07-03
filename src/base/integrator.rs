use crate::pbrt::*;

pub struct SimpleIntegrator {}

impl SimpleIntegrator {
    pub fn new() -> Self {
        return SimpleIntegrator {};
    }

    pub fn evaluate_pixel_sample(
        &self,
        pPixel: Point2i,
        camera: Arc<Mutex<dyn Camera>>,
        shapes: Vec<Arc<dyn Shape>>,
    ) {
        // TODO: rewrite sampler initialization

        let film = camera.lock().unwrap().get_film();

        let filter = film.lock().unwrap().filter.clone();

        let mut sampler = SimpleSampler::new_from_seed(0);

        let camera_sample = get_camera_sample(&mut sampler, pPixel.clone(), filter);

        let camera_ray = camera.lock().unwrap().generate_camera_ray(camera_sample);

        for shape in shapes {
            match shape.intersect(&camera_ray, Float::INFINITY) {
                None => {
                    continue;
                }

                Some(shape_intersection) => {
                    let color = shape_intersection.normal.normalize().softmax_color();

                    film.lock().unwrap().add_sample(pPixel, color);
                    return;
                }
            }
        }
        //panic!("ray missed on {}", pPixel);
        // TODO: 07/01 implementing
    }
}
