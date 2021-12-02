use std::fs;
use std::io::prelude::*;
use crate::vector::*;

pub struct Image {
    pixels: Vec<Vec<Vector>>,
    pub width: usize,
    pub height: usize,
}

impl Image {
    pub fn new(_width: usize, _height: usize) -> Self {
        return Self {
            width: _width,
            height: _height,
            pixels: vec![vec![Color::zero(); _width]; _height],
        };
    }

    pub fn fill(&mut self, rgb_value: Vector, y: usize, x: usize) {
        self.pixels[y][x] = rgb_value;
    }

    pub fn write(&self, ppm_file_name: &str) {
        let ppm_head = format!("P3\n{} {}\n255\n", self.width, self.height);
        fs::write(ppm_file_name, ppm_head)
            .expect(&format!("Failed to write to `{}`", ppm_file_name));

        let mut file = std::fs::OpenOptions::new()
            .append(true)
            .open(ppm_file_name)
            .unwrap();
        let factor = 256 as f32 - 0.001;
        for h in 0usize..self.height {
            for w in 0usize..self.width {
                let pixel = self.pixels[h][w] * factor;
                write!(file, "{} {} {}\n",
                       pixel.x as i32,
                       pixel.y as i32,
                       pixel.z as i32)
                    .expect(&format!("Failed to append to `{}`", ppm_file_name));
            }
        }
    }
}
