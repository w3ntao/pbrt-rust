use crate::pbrt::*;

#[derive(Clone, Copy)]
pub struct SampledSpectrum {
    pub values: [Float; NUM_SPECTRUM_SAMPLES],
}

impl SampledSpectrum {
    pub fn zero() -> Self {
        return Self {
            values: [0.0; NUM_SPECTRUM_SAMPLES],
        };
    }
    pub fn new(values: [Float; NUM_SPECTRUM_SAMPLES]) -> Self {
        return Self { values };
    }

    pub fn safe_div(&self, divisor: &SampledSpectrum) -> SampledSpectrum {
        let mut values = [Float::NAN; NUM_SPECTRUM_SAMPLES];
        for i in 0..NUM_SPECTRUM_SAMPLES {
            values[i] = if divisor[i] == 0.0 {
                0.0
            } else {
                self.values[i] / divisor[i]
            };
        }

        return SampledSpectrum { values };
    }

    pub fn average(&self) -> Float {
        return self.values.iter().sum::<Float>() / (NUM_SPECTRUM_SAMPLES as Float);
    }
}

impl Display for SampledSpectrum {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "[ SampledSpectrum [").expect("");
        for x in self.values {
            write!(f, "{}, ", x).expect("");
        }

        write!(f, "] ]")
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

impl Add<SampledSpectrum> for SampledSpectrum {
    type Output = SampledSpectrum;

    fn add(self, rhs: SampledSpectrum) -> Self::Output {
        let mut values = self.values;
        for i in 0..NUM_SPECTRUM_SAMPLES {
            values[i] += rhs.values[i];
        }

        return SampledSpectrum { values };
    }
}

impl AddAssign<SampledSpectrum> for SampledSpectrum {
    fn add_assign(&mut self, rhs: SampledSpectrum) {
        for i in 0..NUM_SPECTRUM_SAMPLES {
            self.values[i] += rhs.values[i];
        }
    }
}

impl Mul<SampledSpectrum> for SampledSpectrum {
    type Output = SampledSpectrum;

    fn mul(self, rhs: SampledSpectrum) -> Self::Output {
        let mut values = self.values;
        for i in 0..NUM_SPECTRUM_SAMPLES {
            values[i] *= rhs.values[i];
        }

        return SampledSpectrum { values };
    }
}

impl Mul<Float> for SampledSpectrum {
    type Output = SampledSpectrum;

    fn mul(self, rhs: Float) -> Self::Output {
        let mut values = self.values;
        for v in &mut values {
            *v *= rhs;
        }
        return SampledSpectrum { values };
    }
}

impl Mul<SampledSpectrum> for Float {
    type Output = SampledSpectrum;

    fn mul(self, rhs: SampledSpectrum) -> Self::Output {
        return rhs * self;
    }
}
