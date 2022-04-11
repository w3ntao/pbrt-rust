use rand::Rng;

use crate::fundamental::point::Point;
use crate::fundamental::utility::random_zero_to_one;

const POINT_COUNT: usize = 256;

pub struct Perlin {
    random_float: [f32; POINT_COUNT],
    permuted_x: [i32; POINT_COUNT],
    permuted_y: [i32; POINT_COUNT],
    permuted_z: [i32; POINT_COUNT],
}

fn permute(array: &mut [i32]) {
    let length = array.len();
    for idx in (0..length).rev() {
        let target = rand::thread_rng().gen_range(0..length);

        let temp = array[idx];
        array[idx] = array[target];
        array[target] = temp;
    }
}

fn generate_perlin_perm() -> [i32; POINT_COUNT] {
    let mut array = [0; POINT_COUNT];
    for idx in 0..array.len() {
        array[idx] = idx as i32;
    }

    permute(&mut array);
    return array;
}

fn trilinear_interpolate(c: [[[f32; 2]; 2]; 2], u: f32, v: f32, w: f32) -> f32 {
    let mut accumulate = 0.0;
    for i in 0..2 {
        for j in 0..2 {
            for k in 0..2 {
                let value_c = c[i][j][k];
                let i = i as f32;
                let j = j as f32;
                let k = k as f32;

                accumulate +=
                    (i * u + (1.0 - i) * (1.0 - u)) *
                        (j * v + (1.0 - j) * (1.0 - v)) *
                        (k * w + (1.0 - k) * (1.0 - w)) * value_c;
            }
        }
    }

    return accumulate;
}

impl Perlin {
    pub fn new() -> Perlin {
        let mut initialized_random_float: [f32; POINT_COUNT] = [0.0; POINT_COUNT];
        for idx in 0..POINT_COUNT {
            initialized_random_float[idx] = random_zero_to_one();
        }

        return Perlin {
            random_float: initialized_random_float,
            permuted_x: generate_perlin_perm(),
            permuted_y: generate_perlin_perm(),
            permuted_z: generate_perlin_perm(),
        };
    }

    pub fn noise(&self, p: Point) -> f32 {
        let u = p.x - p.x.floor();
        let v = p.y - p.y.floor();
        let w = p.z - p.z.floor();

        let i = p.x.floor() as i32;
        let j = p.y.floor() as i32;
        let k = p.z.floor() as i32;

        let mut c = [[[0.0; 2]; 2]; 2];
        for di in 0..2 {
            for dj in 0..2 {
                for dk in 0..2 {
                    let index =
                        self.permuted_x[((i + di) & 255) as usize] ^ self.permuted_x[((j + dj) & 255) as usize] ^ self.permuted_x[((k + dk) & 255) as usize];

                    c[di as usize][dj as usize][dk as usize] = self.random_float[index as usize];
                }
            }
        }

        return trilinear_interpolate(c, u, v, w);
    }
}
