use crate::pbrt::*;

pub struct SampledSpectrum {
    pub values: [Float; NUM_SPECTRUM_SAMPLES],
}

impl SampledSpectrum {
    pub fn new(values: [Float; NUM_SPECTRUM_SAMPLES]) -> Self {
        return Self { values };
    }
}

impl Index<usize> for SampledSpectrum {
    type Output = Float;

    fn index(&self, index: usize) -> &Self::Output {
        return &self.values[index];
    }
}

impl IndexMut<usize> for SampledSpectrum {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        return &mut self.values[index];
    }
}
