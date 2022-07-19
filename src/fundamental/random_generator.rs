use rand::distributions::{Distribution, Uniform};
use rand::rngs::ThreadRng;
use rand::thread_rng;

pub struct RandomF32Generator {
    generator: ThreadRng,
    distribution: Uniform<f32>,
}

impl RandomF32Generator {
    pub fn new(lower: f32, upper: f32) -> Self {
        return Self {
            generator: thread_rng(),
            distribution: Uniform::new(lower, upper),
        };
    }
    pub fn generate(&mut self) -> f32 {
        return self.distribution.sample(&mut self.generator);
    }
}
