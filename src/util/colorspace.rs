use crate::pbrt::*;

pub struct RGBColorSpace {
    pub r: Point2f,
    pub g: Point2f,
    pub b: Point2f,
    pub w: Point2f,

    pub xyz_from_rgb: SquareMatrix<3>,
    pub rgb_from_xyz: SquareMatrix<3>,

    pub illuminant: DenselySampledSpectrum,
}
