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

    fn preprocess(&mut self, samples_per_pixel: usize, dimensions: usize) {
        *self = RandomSampler::default();
    }

    fn update_round(&mut self) {}

    fn get_camera_ray_sample(&mut self) -> (f32, f32) {
        return (self.rng.gen::<f32>(), self.rng.gen::<f32>());
    }

    fn get_brdf_sample(&mut self) -> (f32, f32) {
        return (self.rng.gen::<f32>(), self.rng.gen::<f32>());
    }
}
