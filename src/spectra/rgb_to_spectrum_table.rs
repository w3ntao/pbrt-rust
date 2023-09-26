use crate::pbrt::*;

const RESOLUTION: usize = RGB_TO_SPECTRUM_RESOLUTION;

pub struct RGBtoSpectrumTable {
    z_nodes: [f32; RESOLUTION],
    coefficients: Vec<Vec<Vec<Vec<Vec<f32>>>>>,
    // TODO: rust bug: allocating large array triggers main thread overflow
    // https://github.com/rust-lang/rust/issues/53827
}

pub enum Gamut {
    SRgb,
}

impl RGBtoSpectrumTable {
    pub fn new(str_gamut: &str) -> Self {
        let gamut = match str_gamut {
            "sRGB" => Gamut::SRgb,
            _ => {
                panic!("gamut `{}` not implemented", str_gamut)
            }
        };

        let (scale, coefficients) = compute_spectrum_table_data(gamut);
        return RGBtoSpectrumTable::build(scale, &coefficients);
    }

    pub fn build(z_nodes: [f32; RESOLUTION], raw_data_coefficients: &[f32]) -> Self {
        if raw_data_coefficients.len() != 3 * RESOLUTION * RESOLUTION * RESOLUTION * 3 {
            panic!("data length mismatched");
        }

        let mut coefficients =
            vec![vec![vec![vec![vec![0.0; 3]; RESOLUTION]; RESOLUTION]; RESOLUTION]; 3];

        let mut idx = 0;
        for max_component in 0..3 {
            for z in 0..RESOLUTION {
                for y in 0..RESOLUTION {
                    for x in 0..RESOLUTION {
                        for c in 0..3 {
                            coefficients[max_component][z][y][x][c] = raw_data_coefficients[idx];
                            idx += 1;
                        }
                    }
                }
            }
        }

        if idx != 3 * RESOLUTION * RESOLUTION * RESOLUTION * 3 {
            panic!("wrong index");
        }

        return RGBtoSpectrumTable {
            z_nodes,
            coefficients,
        };
    }

    fn find_interval(&self, z: f32) -> usize {
        let sz = self.z_nodes.len();
        let mut size = sz - 2;
        let mut first = 1;

        while size > 0 {
            let half = size >> 1;
            let middle = first + half;

            (first, size) = if self.z_nodes[middle] < z {
                (middle + 1, size - (half + 1))
            } else {
                (first, half)
            };
        }

        return clamp_usize(first - 1, 0, sz - 2);
    }

    pub fn eval(&self, rgb: RGB) -> RGBSigmoidPolynomial {
        if rgb[0] == rgb[1] && rgb[1] == rgb[2] {
            return RGBSigmoidPolynomial::new(
                0.0,
                0.0,
                (rgb[0] - 0.5) / (rgb[0] * (1.0 - rgb[0])).sqrt(),
            );
        }

        let max_component = if rgb[0] > rgb[1] {
            if rgb[0] > rgb[2] {
                0
            } else {
                2
            }
        } else {
            if rgb[1] > rgb[2] {
                1
            } else {
                2
            }
        };

        let z = rgb[max_component] as f32;
        let x = rgb[(max_component + 1) % 3] as f32 * (RESOLUTION - 1) as f32 / z;
        let y = rgb[(max_component + 2) % 3] as f32 * (RESOLUTION - 1) as f32 / z;

        // Compute integer indices and offsets for coefficient interpolation

        let xi = (x as usize).min(RESOLUTION - 2);
        let yi = (y as usize).min(RESOLUTION - 2);

        let zi = self.find_interval(z);

        let dx = x - (xi as f32);
        let dy = y - (yi as f32);
        let dz = (z - self.z_nodes[zi]) / (self.z_nodes[zi + 1] - self.z_nodes[zi]);

        let (dx, dy, dz) = (dx as Float, dy as Float, dz as Float);

        // Trilinearly interpolate sigmoid polynomial coefficients _c_
        let mut c = [Float::NAN; 3];
        for i in 0..3 {
            // Define _co_ lambda for looking up sigmoid polynomial coefficients
            let co = |dx: Float, dy: Float, dz: Float| -> Float {
                self.coefficients[max_component][zi + (dz as usize)][yi + (dy as usize)]
                    [xi + (dx as usize)][i] as Float
            };

            c[i] = lerp(
                dz,
                lerp(
                    dy,
                    lerp(dx, co(0.0, 0.0, 0.0), co(1.0, 0.0, 0.0)),
                    lerp(dx, co(0.0, 1.0, 0.0), co(1.0, 1.0, 0.0)),
                ),
                lerp(
                    dy,
                    lerp(dx, co(0.0, 0.0, 1.0), co(1.0, 0.0, 1.0)),
                    lerp(dx, co(0.0, 1.0, 1.0), co(1.0, 1.0, 1.0)),
                ),
            );
        }

        return RGBSigmoidPolynomial::new(c[0], c[1], c[2]);
    }
}
