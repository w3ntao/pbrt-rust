use crate::pbrt::*;

pub struct RGBColorSpace {
    pub r: Point2f,
    pub g: Point2f,
    pub b: Point2f,
    pub w: Point2f,

    pub xyz_from_rgb: SquareMatrix<3>,
    pub rgb_from_xyz: SquareMatrix<3>,

    pub illuminant: &'static dyn Spectrum,

    rgb_to_spectrum_table: RGBtoSpectrumTable,
}

impl RGBColorSpace {
    pub fn new(
        r: Point2f,
        g: Point2f,
        b: Point2f,
        illuminant: &'static dyn Spectrum,
        rgb_to_spectrum_table: RGBtoSpectrumTable,
    ) -> Self {
        let cie_w = illuminant.to_xyz();
        let cie_r = CIEXYZ::from_xy_y(r, 1.0);
        let cie_g = CIEXYZ::from_xy_y(g, 1.0);
        let cie_b = CIEXYZ::from_xy_y(b, 1.0);

        let rgb = SquareMatrix::<3>::new([
            [cie_r.x, cie_g.x, cie_b.x],
            [cie_r.y, cie_g.y, cie_b.y],
            [cie_r.z, cie_g.z, cie_b.z],
        ]);

        let c = rgb.inverse() * cie_w;

        let xyz_from_rgb = rgb * SquareMatrix::<3>::from_diag([c.x, c.y, c.z]);
        let rgb_from_xyz = xyz_from_rgb.inverse();

        return Self {
            r,
            g,
            b,
            w: cie_w.xy(),
            xyz_from_rgb,
            rgb_from_xyz,
            illuminant,
            rgb_to_spectrum_table,
        };
    }

    pub fn to_rgb_coeffs(&self, rgb: RGB) -> RGBSigmoidPolynomial {
        return self
            .rgb_to_spectrum_table
            .eval(rgb.clamp(0.0, f64::INFINITY));
    }

    pub fn generate_albedo_rgb(&self) -> [RGBAlbedoSpectrum; 3] {
        let val = 0.01;

        return [
            RGBAlbedoSpectrum::new(RGB::new(val, 0.0, 0.0)),
            RGBAlbedoSpectrum::new(RGB::new(0.0, val, 0.0)),
            RGBAlbedoSpectrum::new(RGB::new(0.0, 0.0, val)),
        ];
    }
}
