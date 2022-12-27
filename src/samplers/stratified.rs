use crate::core::pbrt::*;

pub struct StratifiedSampler {
    rng: StdRng,
    round: usize,
    camera_ray_sample_idx: usize,
    camera_ray_samples: Vec<Sample2D>,
    brdf_sample_idx: usize,
    brdf_samples: Vec<Vec<Sample2D>>,
    light_num: usize,
    light_id_sample_idx: usize,
    light_id_samples: Vec<Vec<usize>>,
    light_area_sample_idx: usize,
    light_area_samples: Vec<Vec<Sample2D>>,
}

impl Default for StratifiedSampler {
    fn default() -> Self {
        return Self {
            rng: StdRng::from_entropy(),
            round: usize::MAX,
            camera_ray_sample_idx: usize::MAX,
            camera_ray_samples: vec![],
            brdf_sample_idx: usize::MAX,
            brdf_samples: vec![vec![]],
            light_num: usize::MAX,
            light_id_sample_idx: usize::MAX,
            light_id_samples: vec![vec![]],
            light_area_sample_idx: usize::MAX,
            light_area_samples: vec![vec![]],
        };
    }
}

fn generate_stratified_2d_samples(size: usize, rng: &mut StdRng) -> Vec<Sample2D> {
    fn generate_samples(size: usize, rng: &mut StdRng) -> Vec<Sample2D> {
        let sqrt_size = (size as f32).sqrt() as usize;
        let mut samples = vec![];
        let unit = 1.0 / (sqrt_size as f32);
        for idx_x in 0..sqrt_size {
            for idx_y in 0..sqrt_size {
                let x = ((idx_x as f32) + rng.gen::<f32>()) * unit;
                let y = ((idx_y as f32) + rng.gen::<f32>()) * unit;

                samples.push((x, y));
            }
        }
        if size != sqrt_size * sqrt_size {
            samples.append(&mut generate_samples(size - sqrt_size * sqrt_size, rng));
        }
        return samples;
    }

    let mut samples = generate_samples(size, rng);
    samples.shuffle(rng);
    return samples;
}

impl Sampler for StratifiedSampler {
    fn fork(&self) -> Box<dyn Sampler> {
        return Box::new(StratifiedSampler::default());
    }

    fn preprocess(&mut self, samples_per_pixel: usize, dimensions: usize, light_num: usize) {
        // round == samples_per_pixel
        self.rng = StdRng::from_entropy();
        self.round = 0;
        self.light_num = light_num;
        self.camera_ray_sample_idx = 0;
        self.brdf_sample_idx = 0;
        self.light_area_sample_idx = 0;
        self.light_id_sample_idx = 0;

        self.camera_ray_samples = generate_stratified_2d_samples(samples_per_pixel, &mut self.rng);

        self.brdf_samples = vec![vec![(f32::NAN, f32::NAN); dimensions]; samples_per_pixel];
        self.light_area_samples = vec![vec![(f32::NAN, f32::NAN); dimensions]; samples_per_pixel];
        self.light_id_samples = vec![vec![usize::MAX; dimensions]; samples_per_pixel];

        for d in 0..dimensions {
            let brdf_samples = generate_stratified_2d_samples(samples_per_pixel, &mut self.rng);
            let light_area_samples =
                generate_stratified_2d_samples(samples_per_pixel, &mut self.rng);

            let mut light_id_samples: Vec<usize> = (0..light_num).collect();
            while light_id_samples.len() < samples_per_pixel {
                light_id_samples.append(&mut (0..light_num).collect());
            }
            light_id_samples.shuffle(&mut self.rng);

            for round in 0..samples_per_pixel {
                self.brdf_samples[round][d] = brdf_samples[round];
                self.light_id_samples[round][d] = light_id_samples[round];
                self.light_area_samples[round][d] = light_area_samples[round];
            }
        }
    }

    fn update_round(&mut self) {
        self.round += 1;
        self.camera_ray_sample_idx = 0;
        self.brdf_sample_idx = 0;
        self.light_id_sample_idx = 0;
        self.light_area_sample_idx = 0;
    }

    fn get_camera_ray_sample(&mut self) -> Sample2D {
        if self.camera_ray_sample_idx >= self.camera_ray_samples.len() {
            return (self.rng.gen::<f32>(), self.rng.gen::<f32>());
        }

        let sample = self.camera_ray_samples[self.camera_ray_sample_idx];
        self.camera_ray_sample_idx += 1;
        return sample;
    }

    fn get_brdf_sample(&mut self) -> Sample2D {
        if self.round >= self.brdf_samples.len()
            || self.brdf_sample_idx >= self.brdf_samples[self.round].len()
        {
            return (self.rng.gen::<f32>(), self.rng.gen::<f32>());
        }

        let sample = self.brdf_samples[self.round][self.brdf_sample_idx];
        self.brdf_sample_idx += 1;
        return sample;
    }

    fn get_light_id_sample(&mut self) -> usize {
        if self.round >= self.light_id_samples.len()
            || self.light_id_sample_idx >= self.light_id_samples[self.round].len()
        {
            return self.rng.gen_range(0..self.light_num);
        }

        let sample = self.light_id_samples[self.round][self.light_id_sample_idx];
        self.light_id_sample_idx += 1;
        return sample;
    }

    fn get_light_area_sample(&mut self) -> Sample2D {
        if self.round >= self.light_area_samples.len()
            || self.light_area_sample_idx >= self.light_area_samples[self.round].len()
        {
            return (self.rng.gen::<f32>(), self.rng.gen::<f32>());
        }

        let sample = self.light_area_samples[self.round][self.light_area_sample_idx];
        self.light_area_sample_idx += 1;
        return sample;
    }
}
