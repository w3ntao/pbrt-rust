use rand::distributions::{Distribution, Uniform};
use rand::rngs::ThreadRng;
use rand::{thread_rng, Rng};

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

pub fn random_f32(low: f32, high: f32) -> f32 {
    let mut rng = thread_rng();
    let uniform_distribution = Uniform::new(low, high);
    return uniform_distribution.sample(&mut rng);
}

pub fn random_u128() -> u128 {
    let mut rng = thread_rng();
    return rng.gen::<u128>();
}
