use crate::pbrt::*;

#[derive(Clone, Copy)]
pub struct SampledSpectrum {
    pub values: [f64; NUM_SPECTRUM_SAMPLES],
}

impl SampledSpectrum {
    pub const fn same_value(v: f64) -> Self {
        return Self {
            values: [v; NUM_SPECTRUM_SAMPLES],
        };
    }

    pub fn new(values: [f64; NUM_SPECTRUM_SAMPLES]) -> Self {
        debug_assert!(values.into_par_iter().all(|x| x >= 0.0));
        return Self { values };
    }

    pub fn is_positive(&self) -> bool {
        return self.values.into_par_iter().any(|x| x > 0.0);
    }

    pub fn safe_div(&self, divisor: &SampledSpectrum) -> SampledSpectrum {
        let mut values = [f64::NAN; NUM_SPECTRUM_SAMPLES];
        for i in 0..NUM_SPECTRUM_SAMPLES {
            values[i] = if divisor[i] == 0.0 {
                0.0
            } else {
                self.values[i] / divisor[i]
            };
        }

        return SampledSpectrum { values };
    }

    pub fn average(&self) -> f64 {
        return self.values.iter().sum::<f64>() / (NUM_SPECTRUM_SAMPLES as f64);
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
    type Output = f64;

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

impl Mul<f64> for SampledSpectrum {
    type Output = SampledSpectrum;

    fn mul(self, rhs: f64) -> Self::Output {
        let mut values = self.values;
        for v in &mut values {
            *v *= rhs;
        }
        return SampledSpectrum { values };
    }
}

impl Mul<SampledSpectrum> for f64 {
    type Output = SampledSpectrum;

    fn mul(self, rhs: SampledSpectrum) -> Self::Output {
        return rhs * self;
    }
}

impl MulAssign<SampledSpectrum> for SampledSpectrum {
    fn mul_assign(&mut self, rhs: SampledSpectrum) {
        for idx in 0..NUM_SPECTRUM_SAMPLES {
            self.values[idx] *= rhs.values[idx];
        }
    }
}

impl Div<f64> for SampledSpectrum {
    type Output = SampledSpectrum;

    fn div(self, rhs: f64) -> Self::Output {
        let mut values = self.values;
        for v in &mut values {
            *v /= rhs;
        }
        return SampledSpectrum { values };
    }
}
