use crate::pbrt::*;
use image::{ImageBuffer, Rgb, RgbImage};

#[derive(Clone)]
pub struct SimpleRGBFilm {
    pub resolution: Point2i,
    pub filename: String,
    pub filter: Arc<BoxFilter>,
    pixels: Vec<Vec<RGBColor>>,
}

impl SimpleRGBFilm {
    pub fn new(_resolution: Point2i, _filename: &String, _filter: Arc<BoxFilter>) -> Self {
        let width = _resolution.x;
        let height = _resolution.y;

        let postfix = get_postfix(_filename);
        let png_filename = if postfix == "png" {
            _filename.clone()
        } else {
            println!(
                "image format `.{}` is not supported, changed to `.png`",
                postfix
            );
            change_postfix(_filename, "png")
        };

        return SimpleRGBFilm {
            resolution: _resolution.clone(),
            filename: png_filename,
            filter: _filter.clone(),
            pixels: vec![vec![RGBColor::black(); width as usize]; height as usize],
        };
    }

    pub fn add_sample(&mut self, point_film: Point2i, spectrum: RGBColor) {
        self.pixels[point_film.y as usize][point_film.x as usize] = spectrum;
    }

    pub fn save_image(&self) {
        let mut buffer: RgbImage =
            ImageBuffer::new(self.resolution.x as u32, self.resolution.y as u32);
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

        buffer.save(self.filename.clone()).unwrap();
        println!("image saved to `{}`", self.filename.clone());
    }
}
