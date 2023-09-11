use crate::pbrt::*;

pub trait Spectrum {
    fn eval(&self, lambda: Float) -> Float;

    fn non_zero(&self) -> bool;

    fn inner_product(&self, g: &dyn Spectrum) -> Float {
        return (LAMBDA_MIN as usize..(LAMBDA_MAX as usize + 1))
            .map(|_lambda| self.eval(_lambda as Float) * g.eval(_lambda as Float))
            .sum();
    }
}

pub const LAMBDA_MIN: Float = 360.0;
pub const LAMBDA_MAX: Float = 830.0;

pub const NUM_SPECTRUM_SAMPLES: usize = 4;

pub const CIE_Y_INTEGRAL: Float = 106.856895;

pub const CIE_MATCHING_CURVE_Y: ConstPieceWiseLinearSpectrum<NUM_CIE_SAMPLES> =
    ConstPieceWiseLinearSpectrum::new(CIE_LAMBDA, CIE_Y_VALUE);

pub const CIE_MATCHING_CURVE_X: ConstPieceWiseLinearSpectrum<NUM_CIE_SAMPLES> =
    ConstPieceWiseLinearSpectrum::new(CIE_LAMBDA, CIE_X_VALUE);

pub const CIE_MATCHING_CURVE_Z: ConstPieceWiseLinearSpectrum<NUM_CIE_SAMPLES> =
    ConstPieceWiseLinearSpectrum::new(CIE_LAMBDA, CIE_Z_VALUE);

pub const ILLUM_D65: ConstPieceWiseLinearSpectrum<{ CIE_ILLUM_D6500.len() / 2 }> =
    ConstPieceWiseLinearSpectrum::from_interleaved_full_visible_wavelengths(CIE_ILLUM_D6500, true);

pub fn get_named_spectrum(name: &str) -> Arc<dyn Spectrum> {
    return match name {
        "stdillum-D65" => Arc::new(ILLUM_D65),
        _ => {
            panic!("unknown spectrum: `{}`", name);
        }
    };
}

pub fn test_spectrum() {
    // TODO: for debugging only
    let const_illum_d65 = get_named_spectrum("stdillum-D65");

    let non_const_illum_d65 =
        PiecewiseLinearSpectrum::from_interleaved(CIE_ILLUM_D6500.to_vec(), true);
}
