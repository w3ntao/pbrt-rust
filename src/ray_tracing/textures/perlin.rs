use rand::Rng;

use crate::fundamental::color::Color;
use crate::fundamental::point::Point;
use crate::fundamental::utility::random_zero_to_one;
use crate::ray_tracing::texture::Texture;

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
        let x = ((4.0 * p.x) as i32) & 255;
        let y = ((4.0 * p.y) as i32) & 255;
        let z = ((4.0 * p.z) as i32) & 255;

        let idx = self.permuted_x[x as usize] ^ self.permuted_y[y as usize] ^ self.permuted_z[z as usize];
        return self.random_float[idx as usize];
    }
}
