pub type Sample2D = (f32, f32);

pub trait Sampler: Send + Sync {
    fn fork(&self) -> Box<dyn Sampler>;

    fn preprocess(&mut self, samples_per_pixel: usize);

    fn update_round(&mut self);
    // 1 round = 1 ray

    fn get_1d_sample(&mut self) -> f32;

    fn get_2d_sample(&mut self) -> Sample2D;
}

pub const SAMPLES_DIMENSIONS: usize = 32;
// hard code dimensions to 32: we prepare samples only for the first 32 dimensions
// which should be enough for most cases
