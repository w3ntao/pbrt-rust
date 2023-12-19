use crate::pbrt::*;

#[derive(Clone, Copy)]
pub struct SampledWavelengths {
    lambda: [f64; NUM_SPECTRUM_SAMPLES],
    pdf: [f64; NUM_SPECTRUM_SAMPLES],
}

impl Display for SampledWavelengths {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "[ SampledWavelengths - lambda: [").unwrap();

        for lambda in self.lambda {
            write!(f, "{}, ", lambda).unwrap();
        }
        write!(f, "], pdf: [").unwrap();

        for pdf in self.pdf {
            write!(f, "{}, ", pdf).unwrap();
        }
        write!(f, "] ]")
    }
}

impl Index<usize> for SampledWavelengths {
    type Output = f64;

    fn index(&self, index: usize) -> &Self::Output {
        return &self.lambda[index];
    }
}

impl IndexMut<usize> for SampledWavelengths {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        return &mut self.lambda[index];
    }
}

const LAMBDA_EXTEND: f64 = LAMBDA_MAX - LAMBDA_MIN;

const DELTA: f64 = LAMBDA_EXTEND / (NUM_SPECTRUM_SAMPLES as f64);

impl SampledWavelengths {
    pub fn pdf_as_sampled_spectrum(&self) -> SampledSpectrum {
        return SampledSpectrum { values: self.pdf };
    }

    pub fn sample_uniform(u: f64) -> Self {
        let mut lambda = [f64::NAN; NUM_SPECTRUM_SAMPLES];

        lambda[0] = lerp(u, LAMBDA_MIN, LAMBDA_MAX);

        for i in 1..NUM_SPECTRUM_SAMPLES {
            lambda[i] = lambda[i - 1] + DELTA;
            if lambda[i] > LAMBDA_MAX {
                lambda[i] = lambda[i] - LAMBDA_EXTEND;
            }
        }

        let pdf = [1.0 / LAMBDA_EXTEND; NUM_SPECTRUM_SAMPLES];

        debug_assert!(lambda.into_par_iter().all(|x| x >= 0.0));
        debug_assert!(pdf.into_par_iter().all(|x| x >= 0.0));
        debug_assert!(pdf.into_par_iter().any(|x| x > 0.0));

        return Self { lambda, pdf };
    }

    pub fn sample_visible(u: f64) -> Self {
        let mut lambda = [f64::NAN; NUM_SPECTRUM_SAMPLES];
        let mut pdf = [f64::NAN; NUM_SPECTRUM_SAMPLES];
        for i in 0..NUM_SPECTRUM_SAMPLES {
            let up = {
                let up = u + (i as f64) / (NUM_SPECTRUM_SAMPLES as f64);
                if up > 1.0 {
                    up - 1.0
                } else {
                    up
                }
            };

            lambda[i] = sample_visible_wavelengths(up);
            pdf[i] = visible_wavelengths_pdf(lambda[i]);
        }

        return SampledWavelengths { lambda, pdf };
    }
}
