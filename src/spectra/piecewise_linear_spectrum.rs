use crate::pbrt::*;

pub struct PiecewiseLinearSpectrum {
    lambdas: Vec<Float>,
    values: Vec<Float>,
}

impl Spectrum for PiecewiseLinearSpectrum {
    fn eval(&self, lambda: Float) -> Float {
        let size = self.lambdas.len();
        if self.lambdas.len() == 0 || lambda < self.lambdas[0] || lambda > self.lambdas[size - 1] {
            return 0.0;
        }

        let o = find_lambda_interval(lambda, &self.lambdas);

        assert!(lambda >= self.lambdas[o] && lambda <= self.lambdas[o + 1]);

        let t = (lambda - self.lambdas[o]) / (self.lambdas[o + 1] - self.lambdas[o]);

        return lerp(t, self.values[o], self.values[o + 1]);
    }

    fn sample(&self, lambda: &SampledWavelengths) -> SampledSpectrum {
        panic!("not implemented for PiecewiseLinearSpectrum");
    }
}

impl PiecewiseLinearSpectrum {
    pub fn new(lambdas: Vec<Float>, values: Vec<Float>) -> Self {
        assert!(strictly_sorted(&lambdas));
        assert!(finite(&lambdas));
        assert!(finite(&values));

        return Self { lambdas, values };
    }

    pub fn scale(&self, s: Float) -> Self {
        let mut values = self.values.clone();

        for v in &mut values {
            *v *= s;
        }

        return Self {
            lambdas: self.lambdas.clone(),
            values,
        };
    }

    pub fn from_interleaved(samples: Vec<Float>, normalize: bool) -> Self {
        if samples.len() % 2 != 0 {
            panic!("illegal samples number");
        }

        let n = samples.len() / 2;

        let mut lambdas = Vec::<Float>::new();
        let mut values = Vec::<Float>::new();

        if samples[0] > LAMBDA_MIN {
            lambdas.push(LAMBDA_MIN - 1.0);
            values.push(samples[1]);
        }

        for i in 0..n {
            lambdas.push(samples[2 * i]);
            values.push(samples[2 * i + 1]);
        }

        if lambdas[lambdas.len() - 1] < LAMBDA_MAX {
            lambdas.push(LAMBDA_MAX + 1.0);
            values.push(values[values.len() - 1]);
        }

        let spectrum = PiecewiseLinearSpectrum::new(lambdas, values);
        return if !normalize {
            spectrum
        } else {
            return spectrum.scale(CIE_Y_INTEGRAL / spectrum.inner_product(&CIE_Y_PLS));
        };
    }
}
