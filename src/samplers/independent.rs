use crate::pbrt::*;

pub struct IndependentSampler {
    pub(crate) rng: StdRng,
}

impl Default for IndependentSampler {
    fn default() -> Self {
        return IndependentSampler {
            rng: StdRng::from_entropy(),
        };
    }
}

impl IndependentSampler {
    pub fn new_from_seed(seed: u64) -> Self {
        return IndependentSampler {
            rng: StdRng::seed_from_u64(seed),
        };
    }
}

impl Sampler for IndependentSampler {
    fn fork(&self) -> Box<dyn Sampler> {
        return Box::new(IndependentSampler::default());
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
