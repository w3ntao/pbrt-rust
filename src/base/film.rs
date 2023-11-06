use crate::pbrt::*;
use image::{ImageBuffer, Rgb, RgbImage};

pub trait Film: Send + Sync {
    fn fork(&self) -> Box<dyn Film>;

    fn convert_to_rgb_film(&self) -> RGBFilm {
        panic!("you should implement this function only for RGBFilm");
    }

    fn get_filename(&self) -> String;

    fn get_resolution(&self) -> Point2i;

    fn get_filter(&self) -> Arc<dyn Filter>;

    fn get_pixel_rgb(&self, p: Point2i) -> RGB;

    fn merge(&mut self, film: &dyn Film, y_list: Vec<i32>);

    fn add_sample(
        &mut self,
        point_film: Point2i,
        l: &SampledSpectrum,
        lambda: &SampledWavelengths,
        weight: Float,
    );

    fn export_image(&self, filename: &str, resolution: Point2i) {
        let mut buffer: RgbImage = ImageBuffer::new(resolution.x as u32, resolution.y as u32);

        for (x, y, mut_pixel) in buffer.enumerate_pixels_mut() {
            let rgb = self.get_pixel_rgb(Point2i::new(x as i32, y as i32));

            let gamma_rgb = RGB::new(rgb.r.sqrt(), rgb.g.sqrt(), rgb.b.sqrt());
            let u16_rgb = gamma_rgb * (256.0 - 0.0001);

            *mut_pixel = Rgb([u16_rgb.r as u8, u16_rgb.g as u8, u16_rgb.b as u8]);
        }

        buffer.save(filename).unwrap();
        println!("image saved to `{}`", filename);
    }
}
