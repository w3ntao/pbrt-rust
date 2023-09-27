use crate::pbrt::*;

pub struct DenselySampledSpectrum {
    // TODO: remove lambda_min and lambda_max, use const value instead
    lambda_min: usize,
    lambda_max: usize,

    values: Vec<Float>,
}

impl Spectrum for DenselySampledSpectrum {
    fn eval(&self, lambda: Float) -> Float {
        let floor = lambda.floor();
        let ceil = lambda.ceil();

        if floor < self.lambda_min as Float || ceil > self.lambda_max as Float {
            return 0.0;
        }

        // TODO: make a patch for PBRT-v4 regarding this interpolation
        return lerp(
            lambda - floor,
            self.values[floor as usize - self.lambda_min],
            self.values[ceil as usize - self.lambda_min],
        );
    }

    fn sample(&self, lambda: &SampledWavelengths) -> SampledSpectrum {
        // TODO: rewrite this with ConstDenselySampledSpectrum

        let mut values = [Float::NAN; NUM_SPECTRUM_SAMPLES];

        for i in 0..NUM_SPECTRUM_SAMPLES {
            let floor = lambda[i].floor();
            let ceil = lambda[i].ceil();
            values[i] = if floor < self.lambda_min as Float || ceil > self.lambda_max as Float {
                0.0
            } else {
                lerp(
                    lambda[i] - floor as Float,
                    self.values[floor as usize - self.lambda_min],
                    self.values[ceil as usize - self.lambda_min],
                )
            }
        }

        return SampledSpectrum { values };
    }
}

fn sqr(x: Float) -> Float {
    return x * x;
}

impl DenselySampledSpectrum {
    pub fn new(lambda_min: usize, lambda_max: usize) -> Self {
        return Self {
            lambda_min,
            lambda_max,
            values: vec![0.0; lambda_max - lambda_min + 1],
        };
    }

    pub fn from_spectrum(spectrum: &dyn Spectrum, lambda_min: usize, lambda_max: usize) -> Self {
        let mut values = vec![0.0; lambda_max - lambda_min + 1];

        for lambda in lambda_min..(lambda_max + 1) {
            values[lambda - lambda_min] = spectrum.eval(lambda as Float);
        }

        return DenselySampledSpectrum {
            lambda_min,
            lambda_max,
            values,
        };
    }

    pub fn from_sample_function(f: impl Fn(Float) -> Float) -> Self {
        return DenselySampledSpectrum {
            lambda_min: LAMBDA_MIN as usize,
            lambda_max: LAMBDA_MAX as usize,
            values: LAMBDA_RANGE.into_iter().map(f).collect::<Vec<Float>>(),
        };
    }

    // D function in PBRT-v4
    pub fn cie_d(temperature: Float) -> Self {
        let cct = temperature * 1.4338 / 1.4380;
        if cct < 4000.0 {
            // CIE D ill-defined, use blackbody
            let black_body_spectrum = BlackBodySpectrum::new(cct);
            return DenselySampledSpectrum::from_sample_function(|lambda| {
                black_body_spectrum.eval(lambda)
            });
        }

        // Convert CCT to xy
        let x = if cct <= 7000.0 {
            -4.607 * 1e9 / cct.powi(3) + 2.9678 * 1e6 / sqr(cct) + 0.09911 * 1e3 / cct + 0.244063
        } else {
            -2.0064 * 1e9 / cct.powi(3) + 1.9018 * 1e6 / sqr(cct) + 0.24748 * 1e3 / cct + 0.23704
        };

        let y = -3.0 * x * x + 2.870 * x - 0.275;

        // Interpolate D spectrum
        let m = 0.0241 + 0.2562 * x - 0.7341 * y;
        let m1 = (-1.3515 - 1.7703 * x + 5.9114 * y) / m;
        let m2 = (0.0300 - 31.4424 * x + 30.0717 * y) / m;

        let values = (0..N_CIES)
            .into_par_iter()
            .map(|i| (CIE_S0[i] + CIE_S1[i] * m1 + CIE_S2[i] * m2) * 0.01)
            .collect::<Vec<Float>>();

        let piecewise_linear_spectrum = PiecewiseLinearSpectrum::new(CIE_S_LAMBDA.to_vec(), values);

        return DenselySampledSpectrum::from_spectrum(
            &piecewise_linear_spectrum,
            LAMBDA_MIN as usize,
            LAMBDA_MAX as usize,
        );
    }
}
