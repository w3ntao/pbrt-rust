use crate::pbrt::*;

pub struct RGBAlbedoSpectrum {
    rsp: RGBSigmoidPolynomial,
}

impl Spectrum for RGBAlbedoSpectrum {
    fn eval(&self, lambda: Float) -> Float {
        return self.rsp.eval(lambda);
    }

    fn sample(&self, lambda: &SampledWavelengths) -> SampledSpectrum {
        let mut values = [Float::NAN; NUM_SPECTRUM_SAMPLES];
        for i in 0..NUM_SPECTRUM_SAMPLES {
            values[i] = self.rsp.eval(lambda[i]);
        }

        return SampledSpectrum { values };
    }
}

impl RGBAlbedoSpectrum {
    pub fn new(rgb: RGB, color_space: &RGBColorSpace) -> Self {
        return Self {
            rsp: color_space.to_rgb_coeffs(rgb),
        };
    }
}
