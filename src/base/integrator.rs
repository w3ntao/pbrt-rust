use crate::pbrt::*;

pub struct SimpleIntegrator {}

impl SimpleIntegrator {
    pub fn new() -> Self {
        return SimpleIntegrator {};
    }

    pub fn evaluate_pixel_sample(
        &self,
        pPixel: Point2i,
        camera: Arc<Mutex<PerspectiveCamera>>,
        filter: Arc<BoxFilter>,
        shapes: Vec<Triangle>,
    ) {
        // TODO: rewrite sampler initialization
        let mut sampler = SimpleSampler::new_from_seed(0);

        let camera_sample = get_camera_sample(&mut sampler, pPixel, filter);

        let camera_ray = camera.lock().unwrap().generate_camera_ray(camera_sample);

        for shape in shapes {
            match shape.intersect(&camera_ray) {
                None => {
                    continue;
                }
                Some(shape_intersection) => {
                    if pPixel.x % 300 == 0 && pPixel.y % 300 == 0 {
                        println!("{} -> normal: {}", pPixel, shape_intersection.normal);
                    }
                    return;
                }
            }
        }
        //panic!("ray missed on {}", pPixel);
        // TODO: 07/01 implementing
    }
}
