use crate::pbrt::*;

pub struct ConstSpectrum {
    c: f64,
}

impl Spectrum for ConstSpectrum {
    fn eval(&self, _: f64) -> f64 {
        return self.c;
    }

    fn sample(&self, _: &SampledWavelengths) -> SampledSpectrum {
        return SampledSpectrum::same_value(self.c);
    }

    fn is_constant_spectrum(&self) -> bool {
        return true;
    }
}

impl ConstSpectrum {
    pub fn new(c: f64) -> Self {
        return Self { c };
    }
}
