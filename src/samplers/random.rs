use crate::core::pbrt::*;

pub struct RandomSampler {
    rng: StdRng,
    light_num: usize,
}

impl Default for RandomSampler {
    fn default() -> Self {
        return RandomSampler {
            rng: StdRng::from_entropy(),
            light_num: usize::MAX,
        };
    }
}

impl Sampler for RandomSampler {
    fn fork(&self) -> Box<dyn Sampler> {
        return Box::new(RandomSampler::default());
    }

    fn preprocess(&mut self, samples_per_pixel: usize, dimensions: usize, _light_num: usize) {
        *self = RandomSampler::default();
        self.light_num = _light_num;
    }

    fn update_round(&mut self) {}

    fn get_camera_ray_sample(&mut self) -> (f32, f32) {
        return (self.rng.gen::<f32>(), self.rng.gen::<f32>());
    }

    fn get_brdf_sample(&mut self) -> (f32, f32) {
        return (self.rng.gen::<f32>(), self.rng.gen::<f32>());
    }

    fn get_light_id_sample(&mut self) -> usize {
        return self.rng.gen_range(0..self.light_num);
    }

    fn get_light_area_sample(&mut self) -> Sample2D {
        return (self.rng.gen::<f32>(), self.rng.gen::<f32>());
    }
}
