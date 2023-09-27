use crate::pbrt::*;

pub struct BlackBodySpectrum {
    temperature: Float,
    normalization_factor: Float,
}

pub fn black_body(lambda: Float, temperature: Float) -> Float {
    if temperature < 0.0 {
        return 0.0;
    }

    let c: Float = 299792458.0;
    let h: Float = 6.62606957e-34;
    let kb: Float = 1.3806488e-23;
    // Return emitted radiance for blackbody at wavelength _lambda_

    let l = lambda * 1e-9;
    let le = (2.0 * h * c * c) / (l.powi(5) * (((h * c) / (l * kb * temperature)).exp() - 1.0));

    assert!(le.is_finite());
    return le;
}

impl BlackBodySpectrum {
    pub fn new(temperature: Float) -> Self {
        // Compute blackbody normalization constant for given temperature

        let lambda_max = 2.8977721e-3 / temperature;

        return BlackBodySpectrum {
            temperature,
            normalization_factor: 1.0 / black_body(lambda_max * 1e9, temperature),
        };
    }
}

impl Spectrum for BlackBodySpectrum {
    fn eval(&self, lambda: Float) -> Float {
        return black_body(lambda, self.temperature) * self.normalization_factor;
    }

    fn sample(&self, lambda: &SampledWavelengths) -> SampledSpectrum {
        panic!("not implemented");
    }
}
