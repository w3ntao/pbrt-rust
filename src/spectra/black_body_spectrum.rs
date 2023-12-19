use crate::pbrt::*;

pub struct BlackBodySpectrum {
    temperature: f64,
    normalization_factor: f64,
}

pub fn black_body(lambda: f64, temperature: f64) -> f64 {
    if temperature < 0.0 {
        return 0.0;
    }

    let c: f64 = 299792458.0;
    let h: f64 = 6.62606957e-34;
    let kb: f64 = 1.3806488e-23;
    // Return emitted radiance for blackbody at wavelength _lambda_

    let l = lambda * 1e-9;
    let le = (2.0 * h * c * c) / (l.powi(5) * (((h * c) / (l * kb * temperature)).exp() - 1.0));

    assert!(le.is_finite());
    return le;
}

impl BlackBodySpectrum {
    pub fn new(temperature: f64) -> Self {
        // Compute blackbody normalization constant for given temperature

        let lambda_max = 2.8977721e-3 / temperature;

        return BlackBodySpectrum {
            temperature,
            normalization_factor: 1.0 / black_body(lambda_max * 1e9, temperature),
        };
    }
}

impl Spectrum for BlackBodySpectrum {
    fn eval(&self, lambda: f64) -> f64 {
        return black_body(lambda, self.temperature) * self.normalization_factor;
    }

    fn sample(&self, lambda: &SampledWavelengths) -> SampledSpectrum {
        panic!("not implemented");
    }
}
