use crate::core::interfaces::*;
use rand::Rng;

const POINT_COUNT: usize = 256;

pub struct Perlin {
    random_vector: [Vector3; POINT_COUNT],
    permuted_x: [i32; POINT_COUNT],
    permuted_y: [i32; POINT_COUNT],
    permuted_z: [i32; POINT_COUNT],
}

impl Perlin {
    pub fn new() -> Perlin {
        let mut _random_vector = [Vector3::invalid(); POINT_COUNT];
        for idx in 0..POINT_COUNT {
            _random_vector[idx] = Vector3::new(
                random_f32(-1.0, 1.0),
                random_f32(-1.0, 1.0),
                random_f32(-1.0, 1.0),
            )
            .normalize();
        }

        return Perlin {
            random_vector: _random_vector,
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

        let mut c = [[[Vector3::invalid(); 2]; 2]; 2];
        for di in 0..2 {
            for dj in 0..2 {
                for dk in 0..2 {
                    let index = self.permuted_x[((i + di) & 255) as usize]
                        ^ self.permuted_y[((j + dj) & 255) as usize]
                        ^ self.permuted_z[((k + dk) & 255) as usize];

                    c[di as usize][dj as usize][dk as usize] = self.random_vector[index as usize];
                }
            }
        }

        return perlin_interpolate(c, u, v, w);
    }

    pub fn turbulence(&self, point: Point, levels: i32) -> f32 {
        let mut accumulate = 0.0;
        let mut amplitude = 1.0;
        let mut weight = 1.0;

        for _ in 0..levels {
            accumulate += weight * self.noise(point * amplitude);
            amplitude *= 2.0;
            weight *= 0.5;
        }

        return accumulate.abs();
    }
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

fn perlin_interpolate(c: [[[Vector3; 2]; 2]; 2], u: f32, v: f32, w: f32) -> f32 {
    let uu = u * u * (3.0 - 2.0 * u);
    let vv = v * v * (3.0 - 2.0 * v);
    let ww = w * w * (3.0 - 2.0 * w);

    let mut accumulate = 0.0;
    for i in 0..2 {
        for j in 0..2 {
            for k in 0..2 {
                let i = i as f32;
                let j = j as f32;
                let k = k as f32;
                let weight = Vector3::new(u - i, v - j, w - k);

                accumulate += (i * uu + (1.0 - i) * (1.0 - uu))
                    * (j * vv + (1.0 - j) * (1.0 - vv))
                    * (k * ww + (1.0 - k) * (1.0 - ww))
                    * weight.dot(c[i as usize][j as usize][k as usize]);
            }
        }
    }

    return accumulate;
}
