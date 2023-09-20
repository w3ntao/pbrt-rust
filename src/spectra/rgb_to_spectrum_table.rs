use crate::pbrt::*;

const RESOLUTION: usize = RGB_TO_SPECTRUM_RESOLUTION;

pub struct RGBtoSpectrumTable {
    z_nodes: [f32; RESOLUTION],
    coefficients: Vec<Vec<Vec<Vec<Vec<f32>>>>>,
    // TODO: rust bug: allocating large array triggers main thread overflow
    // https://github.com/rust-lang/rust/issues/53827
}

impl RGBtoSpectrumTable {
    pub fn new(z_nodes: [f32; RESOLUTION], raw_data_coefficients: &[f32]) -> Self {
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
}
