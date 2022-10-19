use std::fs;
use std::io::{BufWriter, Write};

use crate::core::color::*;

#[derive(Clone)]
pub struct Image {
    pixels: Vec<Vec<Color>>,
    pub width: usize,
    pub height: usize,
}

impl Image {
    pub fn new(_width: usize, _height: usize) -> Self {
        return Self {
            width: _width,
            height: _height,
            pixels: vec![vec![Color::black(); _width]; _height],
        };
    }

    pub fn fill(&mut self, y: usize, x: usize, rgb_value: Color) {
        self.pixels[y][x] = rgb_value;
    }

    pub fn write(&self, ppm_file_name: &str) {
        let ppm_head = format!("P3\n{} {}\n255\n", self.width, self.height);
        let mut buf_writer = BufWriter::new(fs::File::create(ppm_file_name).unwrap());
        buf_writer.write(ppm_head.as_bytes()).unwrap();

        let factor = 256 as f32 - 0.001;
        for h in 0usize..self.height {
            for w in 0usize..self.width {
                let pixel = &self.pixels[h][w];
                let line = format!(
                    "{} {} {}\n",
                    (pixel.r.sqrt() * factor) as i32,
                    (pixel.g.sqrt() * factor) as i32,
                    (pixel.b.sqrt() * factor) as i32
                );
                // sqrt(): gamma correction
                buf_writer.write(line.as_bytes()).unwrap();
            }
        }
    }
}
