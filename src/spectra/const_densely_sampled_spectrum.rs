use crate::pbrt::*;

const N: usize = (LAMBDA_MAX as usize - LAMBDA_MIN as usize) + 1;

pub struct ConstDenselySampledSpectrum {
    values: [Float; N],
}

impl ConstDenselySampledSpectrum {
    pub const fn from_const_spectrum<const K: usize>(
        spectrum: &ConstPieceWiseLinearSpectrum<K>,
    ) -> Self {
        let mut values = [Float::NAN; N];
        let mut lambda = LAMBDA_MIN as usize;
        while lambda <= LAMBDA_MAX as usize {
            values[lambda - LAMBDA_MIN as usize] = spectrum.const_eval(lambda as Float);
            lambda += 1;
        }

        let mut idx = 0;
        while idx < values.len() {
            assert!(values[idx].is_finite());
            idx += 1;
        }

        return Self { values };
    }
}

impl Spectrum for ConstDenselySampledSpectrum {
    fn eval(&self, lambda: Float) -> Float {
        let floor = lambda.floor();
        let ceil = lambda.ceil();

        if floor < LAMBDA_MIN || ceil > LAMBDA_MAX {
            return 0.0;
        }

        return lerp(
            lambda - floor,
            self.values[floor as usize - LAMBDA_MIN as usize],
            self.values[ceil as usize - LAMBDA_MIN as usize],
        );
    }

    fn sample(&self, lambda: &SampledWavelengths) -> SampledSpectrum {
        // TODO: rewrite this with DenselySampledSpectrum

        let mut values = [Float::NAN; NUM_SPECTRUM_SAMPLES];

        for i in 0..NUM_SPECTRUM_SAMPLES {
            let floor = lambda[i].floor();
            let ceil = lambda[i].ceil();

            values[i] = if floor < LAMBDA_MIN as Float || ceil > LAMBDA_MAX as Float {
                0.0
            } else {
                lerp(
                    lambda[i] - floor as Float,
                    self.values[floor as usize - LAMBDA_MIN as usize],
                    self.values[ceil as usize - LAMBDA_MIN as usize],
                )
            }
        }
        return SampledSpectrum { values };
    }
}
