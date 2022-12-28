use crate::core::pbrt::*;

pub struct RandomSampler {
    rng: StdRng,
}

impl Default for RandomSampler {
    fn default() -> Self {
        return RandomSampler {
            rng: StdRng::from_entropy(),
        };
    }
}

impl Sampler for RandomSampler {
    fn fork(&self) -> Box<dyn Sampler> {
        return Box::new(RandomSampler::default());
    }

    fn preprocess(&mut self, _samples_per_pixel: usize) {
        self.rng = StdRng::from_entropy();
    }

    fn update_round(&mut self) {}

    fn get_1d_sample(&mut self) -> f32 {
        return self.rng.gen::<f32>();
    }

    fn get_2d_sample(&mut self) -> Sample2D {
        return (self.rng.gen::<f32>(), self.rng.gen::<f32>());
    }
}
