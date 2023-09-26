use crate::pbrt::*;

pub struct SampledWavelengths {
    pub lambda: [Float; NUM_SPECTRUM_SAMPLES],
    pub pdf: [Float; NUM_SPECTRUM_SAMPLES],
}

impl Index<usize> for SampledWavelengths {
    type Output = Float;

    fn index(&self, index: usize) -> &Self::Output {
        return &self.lambda[index];
    }
}

impl IndexMut<usize> for SampledWavelengths {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        return &mut self.lambda[index];
    }
}

const LAMBDA_EXTEND: Float = LAMBDA_MAX - LAMBDA_MIN;

const DELTA: Float = LAMBDA_EXTEND / (NUM_SPECTRUM_SAMPLES as Float);

impl SampledWavelengths {
    pub fn sample_uniform(u: Float) -> Self {
        let mut lambda = [Float::NAN; NUM_SPECTRUM_SAMPLES];

        lambda[0] = lerp(u, LAMBDA_MIN, LAMBDA_MAX);

        for i in 1..NUM_SPECTRUM_SAMPLES {
            lambda[i] = lambda[i - 1] + DELTA;
            if lambda[i] > LAMBDA_MAX {
                lambda[i] = lambda[i] - LAMBDA_EXTEND;
            }
        }

        let pdf = [1.0 / LAMBDA_EXTEND; NUM_SPECTRUM_SAMPLES];

        return Self { lambda, pdf };
    }
}
