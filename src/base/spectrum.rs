use crate::pbrt::*;

pub enum SpectrumType {
    Illuminant,
    Albedo,
    Unbounded,
}

pub trait Spectrum: Send + Sync {
    fn eval(&self, lambda: Float) -> Float;

    fn sample(&self, lambda: &SampledWavelengths) -> SampledSpectrum;
}

pub trait SpectrumDefaultInterface: Spectrum {
    fn to_photometric(&self) -> Float {
        // TODO: implement a special case for RGBIlluminantSpectrum
        return self.inner_product(&CIE_Y_DENSELY_SAMPLED);
    }

    fn inner_product(&self, g: &dyn Spectrum) -> Float {
        // The parallel (possibly faster) implementation
        return LAMBDA_RANGE
            .par_iter()
            .map(|_lambda| self.eval(*_lambda) * g.eval(*_lambda))
            .sum();
    }

    fn to_xyz(&self) -> CIEXYZ {
        return CIEXYZ {
            x: self.inner_product(&CIE_X_PLS),
            y: self.inner_product(&CIE_Y_PLS),
            z: self.inner_product(&CIE_Z_PLS),
        } / CIE_Y_INTEGRAL;
    }
}

impl<T: ?Sized + Spectrum> SpectrumDefaultInterface for T {}

pub const LAMBDA_MIN: Float = 360.0;
pub const LAMBDA_MAX: Float = 830.0;

pub const LAMBDA_RANGE: [Float; NUM_CIE_SAMPLES] = {
    let mut lambdas = [Float::NAN; NUM_CIE_SAMPLES];

    let lambda_min_usize = LAMBDA_MIN as usize;

    let mut _lambda = lambda_min_usize;
    while _lambda <= (LAMBDA_MAX as usize) {
        lambdas[_lambda - lambda_min_usize] = _lambda as Float;
        _lambda += 1;
    }
    lambdas
};

const _CHECK_LAMBDA: bool = {
    assert!(LAMBDA_RANGE[0] == LAMBDA_MIN);
    assert!(LAMBDA_RANGE[LAMBDA_RANGE.len() - 1] == LAMBDA_MAX);
    true
};
pub const NUM_SPECTRUM_SAMPLES: usize = 4;

pub const CIE_Y_INTEGRAL: Float = 106.856895;
// TODO: compute CIE_Y_INTEGRAL during compilation rather than hard coding it

pub const CIE_Y_PLS: ConstPieceWiseLinearSpectrum<NUM_CIE_SAMPLES> =
    ConstPieceWiseLinearSpectrum::new(CIE_LAMBDA_RANGE, CIE_Y_VALUE);

pub const CIE_X_PLS: ConstPieceWiseLinearSpectrum<NUM_CIE_SAMPLES> =
    ConstPieceWiseLinearSpectrum::new(CIE_LAMBDA_RANGE, CIE_X_VALUE);

pub const CIE_Z_PLS: ConstPieceWiseLinearSpectrum<NUM_CIE_SAMPLES> =
    ConstPieceWiseLinearSpectrum::new(CIE_LAMBDA_RANGE, CIE_Z_VALUE);

pub const CIE_X_DENSELY_SAMPLED: DenselySampledSpectrum =
    DenselySampledSpectrum::from_const_spectrum(&CIE_X_PLS);

pub const CIE_Y_DENSELY_SAMPLED: DenselySampledSpectrum =
    DenselySampledSpectrum::from_const_spectrum(&CIE_Y_PLS);

pub const CIE_Z_DENSELY_SAMPLED: DenselySampledSpectrum =
    DenselySampledSpectrum::from_const_spectrum(&CIE_Z_PLS);

pub const ILLUM_D65: ConstPieceWiseLinearSpectrum<{ CIE_ILLUM_D6500.len() / 2 }> =
    ConstPieceWiseLinearSpectrum::from_interleaved_full_visible_wavelengths(CIE_ILLUM_D6500, true);

pub fn get_named_spectrum(name: &str) -> &'static dyn Spectrum {
    return match name {
        "stdillum-D65" => &ILLUM_D65,
        _ => {
            panic!("unknown spectrum: `{}`", name);
        }
    };
}
