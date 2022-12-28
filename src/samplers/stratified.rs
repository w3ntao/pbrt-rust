use crate::core::pbrt::*;

pub struct StratifiedSampler {
    rng: StdRng,
    round: usize,
    dimension_1d: usize,
    dimension_2d: usize,
    samples_1d: Vec<Vec<f32>>,
    samples_2d: Vec<Vec<Sample2D>>,
}

impl Default for StratifiedSampler {
    fn default() -> Self {
        return Self {
            rng: StdRng::from_entropy(),
            round: usize::MAX,
            dimension_1d: usize::MAX,
            dimension_2d: usize::MAX,
            samples_1d: vec![],
            samples_2d: vec![vec![]],
        };
    }
}

fn generate_stratified_1d_samples(size: usize, rng: &mut StdRng) -> Vec<f32> {
    let mut samples = vec![];
    let unit = 1.0 / (size as f32);
    for idx in 0..size {
        let val = ((idx as f32) + rng.gen::<f32>()) * unit;
        samples.push(val);
    }
    return samples;
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

    fn preprocess(&mut self, samples_per_pixel: usize) {
        // round == samples_per_pixel
        self.rng = StdRng::from_entropy();
        self.round = 0;

        self.dimension_1d = 0;
        self.dimension_2d = 0;

        self.samples_1d = vec![vec![f32::NAN; SAMPLES_DIMENSIONS]; samples_per_pixel];
        self.samples_2d = vec![vec![(f32::NAN, f32::NAN); SAMPLES_DIMENSIONS]; samples_per_pixel];

        for d in 0..SAMPLES_DIMENSIONS {
            let samples_1d = generate_stratified_1d_samples(samples_per_pixel, &mut self.rng);
            let samples_2d = generate_stratified_2d_samples(samples_per_pixel, &mut self.rng);
            for round in 0..samples_per_pixel {
                self.samples_1d[round][d] = samples_1d[round];
                self.samples_2d[round][d] = samples_2d[round];
            }
        }
    }

    fn update_round(&mut self) {
        self.round += 1;
        self.dimension_1d = 0;
        self.dimension_2d = 0;
    }

    fn get_1d_sample(&mut self) -> f32 {
        if self.round >= self.samples_1d.len()
            || self.dimension_1d >= self.samples_1d[self.round].len()
        {
            return self.rng.gen::<f32>();
        }

        let sample = self.samples_1d[self.round][self.dimension_1d];
        self.dimension_1d += 1;
        return sample;
    }

    fn get_2d_sample(&mut self) -> Sample2D {
        if self.round >= self.samples_2d.len()
            || self.dimension_2d >= self.samples_2d[self.round].len()
        {
            return (self.rng.gen::<f32>(), self.rng.gen::<f32>());
        }

        let sample = self.samples_2d[self.round][self.dimension_2d];
        self.dimension_2d += 1;
        return sample;
    }
}
