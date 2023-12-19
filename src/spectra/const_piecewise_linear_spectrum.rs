use crate::pbrt::*;

pub const fn find_lambda_interval(lambda: f64, lambdas: &[f64]) -> usize {
    let mut size = lambdas.len() - 2;
    let mut first = 1;

    while size > 0 {
        let half = size >> 1;
        let middle = first + half;

        (first, size) = if lambdas[middle] <= lambda {
            (middle + 1, size - (half + 1))
        } else {
            (first, half)
        };
    }

    return clamp_usize(first - 1, 0, lambdas.len() - 2);
}

pub const fn strictly_sorted(arrays: &[f64]) -> bool {
    let mut idx = 0;
    while idx < arrays.len() - 1 {
        if arrays[idx] >= arrays[idx + 1] {
            return false;
        }
        idx += 1;
    }

    return true;
}

pub const fn finite(array: &[f64]) -> bool {
    let mut idx = 0;
    while idx < array.len() {
        if !array[idx].is_finite() {
            return false;
        }
        idx += 1;
    }

    return true;
}

pub struct ConstPieceWiseLinearSpectrum<const N: usize> {
    lambdas: [f64; N],
    values: [f64; N],
}

impl<const N: usize> ConstPieceWiseLinearSpectrum<N> {
    pub const fn new(lambdas: [f64; N], values: [f64; N]) -> Self {
        Self { lambdas, values }
    }

    pub const fn const_eval(&self, lambda: f64) -> f64 {
        if self.lambdas.len() == 0 || lambda < self.lambdas[0] || lambda > self.lambdas[N - 1] {
            return 0.0;
        }

        let o = find_lambda_interval(lambda, &self.lambdas);

        assert!(lambda >= self.lambdas[o] && lambda <= self.lambdas[o + 1]);

        let t = (lambda - self.lambdas[o]) / (self.lambdas[o + 1] - self.lambdas[o]);

        return lerp(t, self.values[o], self.values[o + 1]);
    }

    pub const fn const_scale(&self, s: f64) -> Self {
        let mut values = self.values;

        let mut idx = 0;
        while idx < N {
            values[idx] *= s;
            idx += 1;
        }

        return Self {
            lambdas: self.lambdas,
            values,
        };
    }

    const fn const_inner_product<const K: usize>(
        &self,
        g: &ConstPieceWiseLinearSpectrum<K>,
    ) -> f64 {
        let mut integral = 0.0;

        let mut lambda = LAMBDA_MIN as usize;
        while lambda < LAMBDA_MAX as usize + 1 {
            integral += self.const_eval(lambda as f64) * g.const_eval(lambda as f64);
            lambda += 1;
        }

        return integral;
    }

    const fn build_spectrum(lambdas: [f64; N], values: [f64; N], normalize: bool) -> Self {
        assert!(strictly_sorted(&lambdas));
        assert!(finite(&lambdas));
        assert!(finite(&values));

        let spectrum = ConstPieceWiseLinearSpectrum { lambdas, values };
        return if !normalize {
            spectrum
        } else {
            spectrum.const_scale(CIE_Y_INTEGRAL / spectrum.const_inner_product(&CIE_Y_PLS))
        };
    }

    pub const fn from_interleaved_full_visible_wavelengths(
        samples: [f64; N * 2],
        normalize: bool,
    ) -> Self {
        let _lambda_min = samples[0];
        let _lambda_max = samples[N * 2 - 2];

        assert!(_lambda_min <= LAMBDA_MIN && _lambda_max >= LAMBDA_MAX);

        let mut lambdas = [f64::NAN; N];
        let mut values = [f64::NAN; N];

        let mut idx = 0;
        while idx < N {
            lambdas[idx] = samples[2 * idx];
            values[idx] = samples[2 * idx + 1];
            idx += 1;
        }

        return Self::build_spectrum(lambdas, values, normalize);
    }

    pub const fn from_interleaved_missing_short_and_long_wavelengths(
        samples: [f64; N * 2 - 4],
        normalize: bool,
    ) -> Self {
        let _lambda_min = samples[0];
        let _lambda_max = samples[N * 2 - 6];

        assert!(_lambda_min > LAMBDA_MIN);
        assert!(_lambda_max < LAMBDA_MAX);

        let mut lambdas = [f64::NAN; N];
        let mut values = [f64::NAN; N];

        lambdas[0] = LAMBDA_MIN - 1.0;
        values[0] = samples[1];

        lambdas[N - 1] = LAMBDA_MAX + 1.0;
        values[N - 1] = samples[samples.len() - 1];

        let mut idx = 0;
        while idx < (samples.len() / 2) {
            lambdas[idx + 1] = samples[2 * idx];
            values[idx + 1] = samples[2 * idx + 1];
            idx += 1;
        }

        return Self::build_spectrum(lambdas, values, normalize);
    }
}

impl<const N: usize> Spectrum for ConstPieceWiseLinearSpectrum<N> {
    fn eval(&self, lambda: f64) -> f64 {
        return self.const_eval(lambda);
    }

    fn sample(&self, lambda: &SampledWavelengths) -> SampledSpectrum {
        let mut values = [f64::NAN; NUM_SPECTRUM_SAMPLES];
        for i in 0..NUM_SPECTRUM_SAMPLES {
            values[i] = self.eval(lambda[i]);
        }

        return SampledSpectrum { values };
    }
}
