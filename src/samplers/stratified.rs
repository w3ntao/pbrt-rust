use crate::core::pbrt::*;

pub struct StratifiedSampler {
    rng: StdRng,
    round: usize,
    camera_ray_samples: Vec<(f32, f32)>,
    camera_ray_sample_idx: usize,
    brdf_samples: Vec<Vec<(f32, f32)>>,
    brdf_sample_idx: usize,
    prepared: bool,
}

impl Default for StratifiedSampler {
    fn default() -> Self {
        return Self {
            rng: StdRng::from_entropy(),
            round: 0,
            camera_ray_samples: vec![],
            camera_ray_sample_idx: 0,
            brdf_samples: vec![vec![]],
            brdf_sample_idx: 0,
            prepared: false,
        };
    }
}

fn generate_stratified_2d_samples(size_per_dimension: usize, rng: &mut StdRng) -> Vec<(f32, f32)> {
    let mut samples = vec![];
    let unit = 1.0 / (size_per_dimension as f32);
    for idx_x in 0..size_per_dimension {
        for idx_y in 0..size_per_dimension {
            let x = ((idx_x as f32) + rng.gen::<f32>()) * unit;
            let y = ((idx_y as f32) + rng.gen::<f32>()) * unit;

            samples.push((x, y));
        }
    }
    samples.shuffle(rng);
    return samples;
}

impl Sampler for StratifiedSampler {
    fn fork(&self) -> Box<dyn Sampler> {
        return Box::new(StratifiedSampler::default());
    }

    fn prepare(&mut self, samples_per_pixel: usize, dimensions: usize) {
        // round == samples_per_pixel

        self.prepared = true;
        self.rng = StdRng::from_entropy();

        self.round = 0;
        self.camera_ray_sample_idx = 0;
        self.brdf_sample_idx = 0;

        self.camera_ray_samples = generate_stratified_2d_samples(
            (samples_per_pixel as f32).sqrt() as usize,
            &mut self.rng,
        );

        self.brdf_samples = vec![vec![]; samples_per_pixel];

        for round in 0..samples_per_pixel {
            let stratified_2d_samples =
                generate_stratified_2d_samples((dimensions as f32).sqrt() as usize, &mut self.rng);

            for d in 0..stratified_2d_samples.len() {
                self.brdf_samples[round].push(stratified_2d_samples[d]);
            }
        }
    }

    fn update_round(&mut self) {
        self.round += 1;
    }

    fn get_camera_ray_sample(&mut self) -> (f32, f32) {
        if !self.prepared {
            panic!("StratifiedSampler not prepared");
        }

        if self.camera_ray_sample_idx >= self.camera_ray_samples.len() {
            return (self.rng.gen::<f32>(), self.rng.gen::<f32>());
        }

        let sample = self.camera_ray_samples[self.camera_ray_sample_idx];
        self.camera_ray_sample_idx += 1;
        return sample;
    }

    fn get_brdf_sample(&mut self) -> (f32, f32) {
        if !self.prepared {
            panic!("StratifiedSampler not prepared");
        }

        if self.round >= self.brdf_samples.len()
            || (self.round == self.brdf_samples.len() - 1
                && self.brdf_sample_idx >= self.brdf_samples[self.round].len())
        {
            return (self.rng.gen::<f32>(), self.rng.gen::<f32>());
        }

        let sample = self.brdf_samples[self.round][self.brdf_sample_idx];
        self.brdf_sample_idx += 1;
        return sample;
    }
}
