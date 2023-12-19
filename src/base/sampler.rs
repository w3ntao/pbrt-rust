use crate::pbrt::*;

pub trait Sampler: Send + Sync {
    fn fork(&self) -> Box<dyn Sampler>;

    fn samples_per_pixel(&self) -> usize;

    fn start_pixel_sample(&mut self, p_pixel: Point2i, sample_index: usize);

    fn get_1d(&mut self) -> f64;

    fn get_2d(&mut self) -> Point2f;

    fn get_pixel_2d(&mut self) -> Point2f;

    fn get_camera_sample(&mut self, p_pixel: Point2i, filter: Arc<dyn Filter>) -> CameraSample {
        let fs = filter.sample(self.get_pixel_2d());
        return CameraSample::new(
            Point2f::from(p_pixel) + fs.p + Vector2f::new(0.5, 0.5),
            self.get_2d(),
            1.0,
        );
    }
}
