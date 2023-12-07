use crate::pbrt::*;

pub struct ConstSpectrum {
    c: Float,
}

impl Spectrum for ConstSpectrum {
    fn eval(&self, _: Float) -> Float {
        return self.c;
    }

    fn sample(&self, _: &SampledWavelengths) -> SampledSpectrum {
        return SampledSpectrum::same_value(self.c);
    }
}

impl ConstSpectrum {
    pub fn new(c: Float) -> Self {
        return Self { c };
    }
}
