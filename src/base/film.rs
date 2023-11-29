use crate::pbrt::*;

pub trait Film: Send + Sync {
    fn fork(&self) -> Box<dyn Film>;

    fn convert_to_rgb_film(&self) -> RGBFilm {
        panic!("you should implement this function only for RGBFilm");
    }

    fn get_filename(&self) -> String;

    fn get_resolution(&self) -> Point2i;

    fn get_filter(&self) -> Arc<dyn Filter>;

    fn get_pixel_rgb(&self, p: Point2i) -> RGB;

    fn merge(&mut self, film: &dyn Film, y_list: Vec<usize>);

    fn add_sample(
        &mut self,
        point_film: Point2i,
        l: &SampledSpectrum,
        lambda: &SampledWavelengths,
        weight: Float,
    );

    fn export_image(&self, filename: &str, resolution: Point2i) {
        let mut image = Image::new(resolution, PixelFormat::U256);

        for y in 0..resolution.y {
            for x in 0..resolution.x {
                image[y as usize][x as usize] = self.get_pixel_rgb(Point2i::new(x, y));
            }
        }

        image.export_to_png(filename, true);
        println!("image saved to `{}`", filename);
    }
}
