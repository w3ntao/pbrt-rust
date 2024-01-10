use crate::pbrt::*;

pub struct RGBIlluminantSpectrum {
    scale: f64,
    rsp: RGBSigmoidPolynomial,
    illuminant: &'static dyn Spectrum,
}

impl Display for RGBIlluminantSpectrum {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "scale: {}", self.scale).unwrap();
        write!(f, "rsp: {}", self.rsp).unwrap();
        Ok(())
    }
}

impl Spectrum for RGBIlluminantSpectrum {
    fn eval(&self, lambda: f64) -> f64 {
        return self.scale * self.rsp.eval(lambda) * self.illuminant.eval(lambda);
    }

    fn sample(&self, lambda: &SampledWavelengths) -> SampledSpectrum {
        let mut s = [f64::NAN; NUM_SPECTRUM_SAMPLES];
        for i in 0..NUM_SPECTRUM_SAMPLES {
            s[i] = self.scale * self.rsp.eval(lambda[i]);
        }

        return self.illuminant.sample(lambda) * SampledSpectrum::new(s);
    }

    fn to_photometric(&self) -> f64 {
        // We have to handle RGBIlluminantSpectrum separately here as it's composed of an
        // illuminant spectrum and an RGB multiplier. We only want to consider the
        // illuminant for the sake of this calculation, and we should consider the
        // RGB separately for the purposes of target power/illuminance computation
        // in the lights themselves (but we currently don't)

        return self.illuminant.to_photometric();
    }
}

impl RGBIlluminantSpectrum {
    pub fn new(rgb: RGB) -> Self {
        let illuminant = COLOR_SPACE.illuminant;

        let m = rgb.max_component();
        let scale = 2.0 * m;
        let rsp = COLOR_SPACE.to_rgb_coeffs(if scale > 0.0 {
            rgb / scale
        } else {
            RGB::new(0.0, 0.0, 0.0)
        });

        return Self {
            scale,
            rsp,
            illuminant,
        };
    }
}
