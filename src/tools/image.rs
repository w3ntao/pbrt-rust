use crate::core::pbrt::*;
use image::{ImageBuffer, Rgb, RgbImage};

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

    pub fn fill(&mut self, y: usize, x: usize, color: Color) {
        self.pixels[y][x] = color;
    }

    pub fn write(&self, file_name: &str) {
        let mut buffer: RgbImage = ImageBuffer::new(self.width as u32, self.height as u32);
        let factor = 256.0 - 0.0001;
        for (x, y, mut_pixel) in buffer.enumerate_pixels_mut() {
            let color = &self.pixels[y as usize][x as usize];
            *mut_pixel = Rgb([
                (color.r.sqrt() * factor) as u8,
                (color.g.sqrt() * factor) as u8,
                (color.b.sqrt() * factor) as u8,
            ]);
            // sqrt(): gamma correction
        }

        buffer.save(format!("{}.png", file_name)).unwrap();
    }
}
