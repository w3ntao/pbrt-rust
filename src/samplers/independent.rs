use crate::pbrt::*;

pub struct IndependentSampler {
    samples_per_pixel: usize,
    pub rng: StdRng,
}

impl IndependentSampler {
    pub fn new(samples_per_pixel: usize) -> Self {
        return Self {
            samples_per_pixel,
            rng: StdRng::from_entropy(),
        };
    }

    pub fn new_from_seed(seed: u64, samples_per_pixel: usize) -> Self {
        return Self {
            samples_per_pixel,
            rng: StdRng::seed_from_u64(seed),
        };
    }
}

impl Sampler for IndependentSampler {
    fn fork(&self) -> Box<dyn Sampler> {
        return Box::new(IndependentSampler::new(self.samples_per_pixel));
    }

    fn samples_per_pixel(&self) -> usize {
        return self.samples_per_pixel;
    }
    fn start_pixel_sample(&mut self, p_pixel: Point2i, sample_index: usize) {
        // do nothing
    }

    fn get_1d(&mut self) -> Float {
        return self.rng.gen::<Float>();
    }

    fn get_2d(&mut self) -> Point2f {
        return Point2f::new(self.rng.gen::<Float>(), self.rng.gen::<Float>());
    }

    fn get_pixel_2d(&mut self) -> Point2f {
        return self.get_2d();
    }
}
