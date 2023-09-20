use crate::pbrt::*;
use std::thread;
use std::thread::JoinHandle;

const CIE_SAMPLES: usize = 95;
const CIE_FINE_SAMPLES: usize = (CIE_SAMPLES - 1) * 3 + 1;

const RGB2SPEC_EPSILON: f64 = 1e-4;

pub const RGB_TO_SPECTRUM_RESOLUTION: usize = 64;
const RESOLUTION: usize = RGB_TO_SPECTRUM_RESOLUTION;

#[rustfmt::skip]
const CIE_X: [f64; CIE_SAMPLES] = [
    0.000129900000, 0.000232100000, 0.000414900000, 0.000741600000, 0.001368000000,
    0.002236000000, 0.004243000000, 0.007650000000, 0.014310000000, 0.023190000000,
    0.043510000000, 0.077630000000, 0.134380000000, 0.214770000000, 0.283900000000,
    0.328500000000, 0.348280000000, 0.348060000000, 0.336200000000, 0.318700000000,
    0.290800000000, 0.251100000000, 0.195360000000, 0.142100000000, 0.095640000000,
    0.057950010000, 0.032010000000, 0.014700000000, 0.004900000000, 0.002400000000,
    0.009300000000, 0.029100000000, 0.063270000000, 0.109600000000, 0.165500000000,
    0.225749900000, 0.290400000000, 0.359700000000, 0.433449900000, 0.512050100000,
    0.594500000000, 0.678400000000, 0.762100000000, 0.842500000000, 0.916300000000,
    0.978600000000, 1.026300000000, 1.056700000000, 1.062200000000, 1.045600000000,
    1.002600000000, 0.938400000000, 0.854449900000, 0.751400000000, 0.642400000000,
    0.541900000000, 0.447900000000, 0.360800000000, 0.283500000000, 0.218700000000,
    0.164900000000, 0.121200000000, 0.087400000000, 0.063600000000, 0.046770000000,
    0.032900000000, 0.022700000000, 0.015840000000, 0.011359160000, 0.008110916000,
    0.005790346000, 0.004109457000, 0.002899327000, 0.002049190000, 0.001439971000,
    0.000999949300, 0.000690078600, 0.000476021300, 0.000332301100, 0.000234826100,
    0.000166150500, 0.000117413000, 0.000083075270, 0.000058706520, 0.000041509940,
    0.000029353260, 0.000020673830, 0.000014559770, 0.000010253980, 0.000007221456,
    0.000005085868, 0.000003581652, 0.000002522525, 0.000001776509, 0.000001251141,
];

#[rustfmt::skip]
const CIE_Y: [f64; CIE_SAMPLES] = [
    0.000003917000, 0.000006965000, 0.000012390000, 0.000022020000, 0.000039000000,
    0.000064000000, 0.000120000000, 0.000217000000, 0.000396000000, 0.000640000000,
    0.001210000000, 0.002180000000, 0.004000000000, 0.007300000000, 0.011600000000,
    0.016840000000, 0.023000000000, 0.029800000000, 0.038000000000, 0.048000000000,
    0.060000000000, 0.073900000000, 0.090980000000, 0.112600000000, 0.139020000000,
    0.169300000000, 0.208020000000, 0.258600000000, 0.323000000000, 0.407300000000,
    0.503000000000, 0.608200000000, 0.710000000000, 0.793200000000, 0.862000000000,
    0.914850100000, 0.954000000000, 0.980300000000, 0.994950100000, 1.000000000000,
    0.995000000000, 0.978600000000, 0.952000000000, 0.915400000000, 0.870000000000,
    0.816300000000, 0.757000000000, 0.694900000000, 0.631000000000, 0.566800000000,
    0.503000000000, 0.441200000000, 0.381000000000, 0.321000000000, 0.265000000000,
    0.217000000000, 0.175000000000, 0.138200000000, 0.107000000000, 0.081600000000,
    0.061000000000, 0.044580000000, 0.032000000000, 0.023200000000, 0.017000000000,
    0.011920000000, 0.008210000000, 0.005723000000, 0.004102000000, 0.002929000000,
    0.002091000000, 0.001484000000, 0.001047000000, 0.000740000000, 0.000520000000,
    0.000361100000, 0.000249200000, 0.000171900000, 0.000120000000, 0.000084800000,
    0.000060000000, 0.000042400000, 0.000030000000, 0.000021200000, 0.000014990000,
    0.000010600000, 0.000007465700, 0.000005257800, 0.000003702900, 0.000002607800,
    0.000001836600, 0.000001293400, 0.000000910930, 0.000000641530, 0.000000451810,
];

#[rustfmt::skip]
const CIE_Z: [f64; CIE_SAMPLES] = [
    0.000606100000, 0.001086000000, 0.001946000000, 0.003486000000, 0.006450001000,
    0.010549990000, 0.020050010000, 0.036210000000, 0.067850010000, 0.110200000000,
    0.207400000000, 0.371300000000, 0.645600000000, 1.039050100000, 1.385600000000,
    1.622960000000, 1.747060000000, 1.782600000000, 1.772110000000, 1.744100000000,
    1.669200000000, 1.528100000000, 1.287640000000, 1.041900000000, 0.812950100000,
    0.616200000000, 0.465180000000, 0.353300000000, 0.272000000000, 0.212300000000,
    0.158200000000, 0.111700000000, 0.078249990000, 0.057250010000, 0.042160000000,
    0.029840000000, 0.020300000000, 0.013400000000, 0.008749999000, 0.005749999000,
    0.003900000000, 0.002749999000, 0.002100000000, 0.001800000000, 0.001650001000,
    0.001400000000, 0.001100000000, 0.001000000000, 0.000800000000, 0.000600000000,
    0.000340000000, 0.000240000000, 0.000190000000, 0.000100000000, 0.000049999990,
    0.000030000000, 0.000020000000, 0.000010000000, 0.000000000000, 0.000000000000,
    0.000000000000, 0.000000000000, 0.000000000000, 0.000000000000, 0.000000000000,
    0.000000000000, 0.000000000000, 0.000000000000, 0.000000000000, 0.000000000000,
    0.000000000000, 0.000000000000, 0.000000000000, 0.000000000000, 0.000000000000,
    0.000000000000, 0.000000000000, 0.000000000000, 0.000000000000, 0.000000000000,
    0.000000000000, 0.000000000000, 0.000000000000, 0.000000000000, 0.000000000000,
    0.000000000000, 0.000000000000, 0.000000000000, 0.000000000000, 0.000000000000,
    0.000000000000, 0.000000000000, 0.000000000000, 0.000000000000, 0.000000000000,
];

#[rustfmt::skip]
const CIE_D65: [f64; CIE_SAMPLES] = {
    const fn f(x: f64) -> f64 {
        return x / 10566.864005283874576;
    }
    [
        f(46.6383), f(49.3637), f(52.0891), f(51.0323), f(49.9755), f(52.3118), f(54.6482),
        f(68.7015), f(82.7549), f(87.1204), f(91.486),  f(92.4589), f(93.4318), f(90.057),
        f(86.6823), f(95.7736), f(104.865), f(110.936), f(117.008), f(117.41),  f(117.812),
        f(116.336), f(114.861), f(115.392), f(115.923), f(112.367), f(108.811), f(109.082),
        f(109.354), f(108.578), f(107.802), f(106.296), f(104.79),  f(106.239), f(107.689),
        f(106.047), f(104.405), f(104.225), f(104.046), f(102.023), f(100.0),   f(98.1671),
        f(96.3342), f(96.0611), f(95.788),  f(92.2368), f(88.6856), f(89.3459), f(90.0062),
        f(89.8026), f(89.5991), f(88.6489), f(87.6987), f(85.4936), f(83.2886), f(83.4939),
        f(83.6992), f(81.863),  f(80.0268), f(80.1207), f(80.2146), f(81.2462), f(82.2778),
        f(80.281),  f(78.2842), f(74.0027), f(69.7213), f(70.6652), f(71.6091), f(72.979),
        f(74.349),  f(67.9765), f(61.604),  f(65.7448), f(69.8856), f(72.4863), f(75.087),
        f(69.3398), f(63.5927), f(55.0054), f(46.4182), f(56.6118), f(66.8054), f(65.0941),
        f(63.3828), f(63.8434), f(64.304),  f(61.8779), f(59.4519), f(55.7054), f(51.959),
        f(54.6998), f(57.4406), f(58.8765), f(60.3125),
    ]
};

const XYZ_TO_SRGB: [[f64; 3]; 3] = [
    [3.240479, -1.537150, -0.498535],
    [-0.969256, 1.875991, 0.041556],
    [0.055648, -0.204043, 1.057311],
];

const SRGB_TO_XYZ: [[f64; 3]; 3] = [
    [0.412453, 0.357580, 0.180423],
    [0.212671, 0.715160, 0.072169],
    [0.019334, 0.119193, 0.950227],
];

enum Gamut {
    SRgb,
}

const fn clamp_usize(val: usize, low: usize, high: usize) -> usize {
    if val < low {
        return low;
    }
    if val > high {
        return high;
    }
    return val;
}

const fn cie_interpolate(data: &[f64], x: f64) -> f64 {
    let x = (x - CIE_LAMBDA_MIN) * ((CIE_SAMPLES as f64 - 1.0) / (CIE_LAMBDA_MAX - CIE_LAMBDA_MIN));
    let offset = clamp_usize(x as usize, 0, CIE_SAMPLES - 2);

    let weight = x - (offset as f64);

    return (1.0 - weight) * data[offset] + weight * data[offset + 1];
}

#[derive(Copy, Clone)]
struct Table {
    lambda_tbl: [f64; CIE_FINE_SAMPLES],
    rgb_tbl: [[f64; CIE_FINE_SAMPLES]; 3],
    rgb_to_xyz: [[f64; 3]; 3],
    xyz_to_rgb: [[f64; 3]; 3],
    xyz_white_point: [f64; 3],
}

const fn init_table(gamut: Gamut) -> Table {
    let (illuminant, xyz_to_rgb, rgb_to_xyz) = match gamut {
        Gamut::SRgb => (CIE_D65, XYZ_TO_SRGB, SRGB_TO_XYZ),
    };

    let h = (CIE_LAMBDA_MAX - CIE_LAMBDA_MIN) / ((CIE_FINE_SAMPLES - 1) as f64);

    let mut i = 0;
    let mut lambda_tbl = [0.0; CIE_FINE_SAMPLES];

    let mut rgb_tbl = [[0.0; CIE_FINE_SAMPLES]; 3];

    let mut xyz_white_point = [0.0; 3];

    while i < CIE_FINE_SAMPLES {
        let lambda = CIE_LAMBDA_MIN + (i as f64) * h;

        let xyz = [
            cie_interpolate(&CIE_X, lambda),
            cie_interpolate(&CIE_Y, lambda),
            cie_interpolate(&CIE_Z, lambda),
        ];
        let big_i = cie_interpolate(&illuminant, lambda);

        let weight = {
            3.0 / 8.0
                * h
                * if i == 0 || i == CIE_FINE_SAMPLES - 1 {
                    1.0
                } else if (i - 1) % 3 == 2 {
                    2.0
                } else {
                    3.0
                }
        };

        lambda_tbl[i] = lambda;
        let mut k = 0;
        while k < 3 {
            let mut j = 0;
            while j < 3 {
                rgb_tbl[k][i] += xyz_to_rgb[k][j] * xyz[j] * big_i * weight;

                j += 1;
            }
            k += 1;
        }

        let mut w = 0;
        while w < 3 {
            xyz_white_point[w] += xyz[w] * big_i * weight;
            w += 1;
        }

        i += 1;
    }

    return Table {
        lambda_tbl,
        rgb_tbl,
        rgb_to_xyz,
        xyz_to_rgb,
        xyz_white_point,
    };
}

fn sigmoid(x: f64) -> f64 {
    // TODO: rust bug: f32.sqrt() can't be made const
    // https://github.com/rust-lang/rust/issues/57241
    return 0.5 * x / (1.0 + x * x).sqrt() + 0.5;
}
fn smooth_step(x: f64) -> f64 {
    return x * x * (3.0 - 2.0 * x);
}

fn cie_lab(table: &Table, p: &mut [f64; 3]) {
    let mut x = 0.0;
    let mut y = 0.0;
    let mut z = 0.0;

    let xw = table.xyz_white_point[0];
    let yw = table.xyz_white_point[1];
    let zw = table.xyz_white_point[2];

    for j in 0..3 {
        x += p[j] * table.rgb_to_xyz[0][j];
        y += p[j] * table.rgb_to_xyz[1][j];
        z += p[j] * table.rgb_to_xyz[2][j];
    }
    let f = |t: f64| -> f64 {
        let delta = 6.0 / 29.0;
        return if t > delta * delta * delta {
            t.cbrt()
        } else {
            t / (delta * delta * 3.0) + (4.0 / 29.0)
        };
    };

    p[0] = 116.0 * f(y / yw) - 16.0;
    p[1] = 500.0 * (f(x / xw) - f(y / yw));
    p[2] = 200.0 * (f(y / yw) - f(z / zw));
}

fn eval_residual(table: &Table, coeffs: &[f64; 3], rgb: &[f64; 3], residual: &mut [f64; 3]) {
    let mut out = [0.0; 3];

    for i in 0..CIE_FINE_SAMPLES {
        /* Scale lambda to 0..1 range */
        let lambda = (table.lambda_tbl[i] - CIE_LAMBDA_MIN) / (CIE_LAMBDA_MAX - CIE_LAMBDA_MIN);

        /* Polynomial */
        let mut x = 0.0;
        for inner_i in 0..3 {
            x = x * lambda + coeffs[inner_i];
        }

        /* Sigmoid */
        let s = sigmoid(x);

        /* Integrate against precomputed curves */
        for j in 0..3 {
            out[j] += table.rgb_tbl[j][i] * s;
        }
    }

    cie_lab(table, &mut out);
    *residual = rgb.clone();
    cie_lab(table, residual);

    for j in 0..3 {
        residual[j] -= out[j];
    }
}

fn eval_jacobian(
    table: &Table,
    coefficients: &[f64; 3],
    rgb: &[f64; 3],
    jacobian: &mut [[f64; 3]; 3],
) {
    let mut r0 = [0.0; 3];
    let mut r1 = [0.0; 3];

    for i in 0..3 {
        let mut tmp = coefficients.clone();
        tmp[i] -= RGB2SPEC_EPSILON;
        eval_residual(table, &tmp, rgb, &mut r0);

        let mut tmp = coefficients.clone();
        tmp[i] += RGB2SPEC_EPSILON;
        eval_residual(table, &tmp, rgb, &mut r1);

        for j in 0..3 {
            jacobian[j][i] = (r1[j] - r0[j]) * 1.0 / (2.0 * RGB2SPEC_EPSILON);
        }
    }
}

fn lup_decompose(a: &mut [[f64; 3]; 3], n: usize, tolerance: f64, p: &mut [usize; 4]) -> bool {
    for i in 0..n + 1 {
        // Unit permutation matrix, P[N] initialized with N
        p[i] = i;
    }

    for i in 0..n {
        let mut max_a = 0.0;
        let mut imax = i;

        for k in i..n {
            let abs_a = a[k][i].abs();
            if abs_a > max_a {
                max_a = abs_a;
                imax = k;
                //println!("imax: {}", imax);
            }
        }

        //println!("maxA: {}", maxA);
        if max_a < tolerance {
            // failure, matrix is degenerate
            return false;
        }

        if imax != i {
            // pivoting P
            let j = p[i];
            p[i] = p[imax];
            p[imax] = j;

            // pivoting rows of A
            let ptr = a[i];
            a[i] = a[imax];
            a[imax] = ptr;

            // counting pivots starting from N (for determinant)
            p[n] += 1;
        }

        for j in i + 1..n {
            a[j][i] /= a[i][i];

            for k in i + 1..n {
                a[j][k] -= a[j][i] * a[i][k];
            }
        }
    }

    return true;
}

fn lup_solve(a: &[[f64; 3]; 3], p: &[usize; 4], b: &[f64; 3], n: usize, x: &mut [f64; 3]) {
    for i in 0..n {
        x[i] = b[p[i]];

        for k in 0..i {
            x[i] -= a[i][k] * x[k];
        }
    }

    for i in (0..n).rev() {
        for k in i + 1..n {
            x[i] -= a[i][k] * x[k];
        }
        x[i] = x[i] / a[i][i];
    }
}

fn gauss_newton(table: &Table, rgb: &[f64; 3], coefficients: &mut [f64; 3]) {
    let iteration = 15;

    for _ in 0..iteration {
        let mut jacobian = [[0.0; 3]; 3];
        let mut residual = [0.0; 3];

        eval_residual(table, coefficients, rgb, &mut residual);
        eval_jacobian(table, coefficients, rgb, &mut jacobian);

        let mut p = [0; 4];
        if !lup_decompose(&mut jacobian, 3, 1e-15, &mut p) {
            panic!("LU decomposition failed!");
        }

        let mut x = [0.0; 3];
        lup_solve(&jacobian, &p, &residual, 3, &mut x);

        let mut r = 0.0;
        for j in 0..3 {
            coefficients[j] -= x[j];
            r += sqr(residual[j]);
        }

        let max = coefficients[0].max(coefficients[1]).max(coefficients[2]);

        if max > 200.0 {
            for j in 0..3 {
                coefficients[j] *= 200.0 / max;
            }
        }

        if r < 1e-6 {
            break;
        }
    }
}

const fn sqr(x: f64) -> f64 {
    return x * x;
}

fn single_thread_iterate(
    spectrum_table_data: &mut Arc<Mutex<SpectrumTableData>>,
    job_list: &mut Arc<Mutex<Vec<(usize, usize)>>>,
    table: Table,
    scale: [f32; RESOLUTION],
) {
    let c0 = CIE_LAMBDA_MIN;
    let c1 = 1.0 / (CIE_LAMBDA_MAX - CIE_LAMBDA_MIN);
    let start = RESOLUTION / 5;

    let mut cache = HashMap::<usize, f32>::new();

    loop {
        let mut locked_job_list = job_list.lock().unwrap();
        let maybe_job = locked_job_list.pop();
        drop(locked_job_list);

        match maybe_job {
            None => {
                break;
            }
            Some((l, j)) => {
                let y = j as f64 / (RESOLUTION - 1) as f64;
                for i in 0..RESOLUTION {
                    let x = i as f64 / (RESOLUTION - 1) as f64;

                    for range in [
                        (start..RESOLUTION).collect::<Vec<usize>>(),
                        (0..start + 1).rev().collect::<Vec<usize>>(),
                    ] {
                        let mut coefficients = [0.0; 3];
                        let mut rgb = [0.0; 3];

                        for k in range {
                            {
                                let b = scale[k] as f64;
                                rgb[l] = b;
                                rgb[(l + 1) % 3] = x * b;
                                rgb[(l + 2) % 3] = y * b;
                            }

                            gauss_newton(&table, &rgb, &mut coefficients);

                            let [a, b, c] = coefficients;

                            let idx = ((l * RESOLUTION + k) * RESOLUTION + j) * RESOLUTION + i;

                            cache.insert(3 * idx + 0, (a * (sqr(c1))) as f32);
                            cache.insert(3 * idx + 1, (b * c1 - 2.0 * a * c0 * (sqr(c1))) as f32);
                            cache
                                .insert(3 * idx + 2, (c - b * c0 * c1 + a * (sqr(c0 * c1))) as f32);
                        }
                    }
                }
            }
        }
    }

    spectrum_table_data.lock().unwrap().add_data(cache);
}

struct SpectrumTableData {
    data: Vec<f32>,
    // TODO: rust bug: allocating large array triggers main thread overflow
    // https://github.com/rust-lang/rust/issues/53827
}

impl SpectrumTableData {
    pub fn new(size: usize) -> Self {
        return Self {
            data: vec![0.0; size],
        };
    }
    pub fn add_data(&mut self, data_map: HashMap<usize, f32>) {
        for idx in data_map.keys() {
            self.data[*idx] = data_map[idx];
        }
    }
}

pub fn compute_spectrum_table_data(str_gamut: &str) -> ([f32; RESOLUTION], Vec<f32>) {
    let gamut = match str_gamut {
        "sRGB" => Gamut::SRgb,
        _ => {
            panic!("gamut `{}` not implemented", str_gamut)
        }
    };

    let table = init_table(gamut);

    let mut scale = [0.0; RESOLUTION];
    for k in 0..RESOLUTION {
        scale[k] = smooth_step(smooth_step(k as f64 / (RESOLUTION - 1) as f64)) as f32;
    }

    const BUFFER_SIZE: usize = 3 * 3 * RESOLUTION * RESOLUTION * RESOLUTION;

    let mut job_list = vec![];
    for l in 0..3 {
        for j in 0..RESOLUTION {
            job_list.push((l, j));
        }
    }
    let shared_job_list = Arc::new(Mutex::new(job_list));
    let spectrum_table_data = Arc::new(Mutex::new(SpectrumTableData::new(BUFFER_SIZE)));

    let mut handles: Vec<JoinHandle<()>> = vec![];
    for _ in 0..num_cpus::get() {
        let mut forked_job_list = shared_job_list.clone();
        let mut forked_spectrum_table_data = spectrum_table_data.clone();

        let handle = thread::spawn(move || {
            single_thread_iterate(
                &mut forked_spectrum_table_data,
                &mut forked_job_list,
                table,
                scale,
            )
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    return (scale, spectrum_table_data.lock().unwrap().data.clone());
}
