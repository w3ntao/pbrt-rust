use crate::pbrt::*;

pub struct DenselySampledSpectrum {
    lambda_min: usize,
    lambda_max: usize,

    values: Vec<Float>,
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

        if spectrum.non_zero() {
            for lambda in lambda_min..(lambda_max + 1) {
                values[lambda - lambda_min] = spectrum.eval(lambda as Float);
            }
        }

        return DenselySampledSpectrum {
            lambda_min,
            lambda_max,
            values,
        };
    }
}
