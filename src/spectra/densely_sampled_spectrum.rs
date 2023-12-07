use crate::pbrt::*;

const N: usize = (LAMBDA_MAX as usize - LAMBDA_MIN as usize) + 1;

pub struct DenselySampledSpectrum {
    values: [Float; N],
}

impl Display for DenselySampledSpectrum {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "[").expect("");
        for x in self.values {
            write!(f, "{}, ", x).expect("");
        }
        write!(f, "]").expect("");

        Ok(())
    }
}

impl Spectrum for DenselySampledSpectrum {
    fn eval(&self, lambda: Float) -> Float {
        let floor = lambda.floor();
        let ceil = lambda.ceil();

        if floor < LAMBDA_MIN || ceil > LAMBDA_MAX as Float {
            return 0.0;
        }

        // TODO: make a patch for PBRT-v4 regarding this interpolation
        return lerp(
            lambda - floor,
            self.values[floor as usize - LAMBDA_MIN as usize],
            self.values[ceil as usize - LAMBDA_MIN as usize],
        );
    }

    fn sample(&self, lambda: &SampledWavelengths) -> SampledSpectrum {
        let mut values = [Float::NAN; NUM_SPECTRUM_SAMPLES];

        for i in 0..NUM_SPECTRUM_SAMPLES {
            let floor = lambda[i].floor();
            let ceil = lambda[i].ceil();
            values[i] = if floor < LAMBDA_MIN as Float || ceil > LAMBDA_MAX as Float {
                0.0
            } else {
                lerp(
                    lambda[i] - floor,
                    self.values[floor as usize - LAMBDA_MIN as usize],
                    self.values[ceil as usize - LAMBDA_MIN as usize],
                )
            }
        }

        return SampledSpectrum { values };
    }
}

impl DenselySampledSpectrum {
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

    pub fn from_spectrum(spectrum: &dyn Spectrum) -> Self {
        let mut values = [Float::NAN; N];

        for lambda in LAMBDA_RANGE {
            values[lambda as usize - LAMBDA_MIN as usize] = spectrum.eval(lambda);
        }

        return DenselySampledSpectrum { values };
    }

    pub fn from_sample_function(f: impl Fn(Float) -> Float) -> Self {
        let mut values = [Float::NAN; N];

        for lambda in LAMBDA_RANGE {
            values[lambda as usize - LAMBDA_MIN as usize] = f(lambda);
        }

        return DenselySampledSpectrum { values };
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

        return DenselySampledSpectrum::from_spectrum(&piecewise_linear_spectrum);
    }
}
