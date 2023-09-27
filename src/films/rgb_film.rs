use crate::pbrt::*;
use image::{ImageBuffer, Rgb, RgbImage};

#[derive(Copy, Clone)]
struct Pixel {
    pub rgb_sum: [f64; 3],
    pub weight_sum: f64,
}

impl Default for Pixel {
    fn default() -> Self {
        return Pixel {
            rgb_sum: [0.0; 3],
            weight_sum: 0.0,
        };
    }
}

#[derive(Clone)]
pub struct RGBFilm {
    pub resolution: Point2i,
    pub filename: String,
    pub filter: Arc<dyn Filter>,
    pub sensor: Arc<PixelSensor>,
    output_rgb_from_sensor_rgb: SquareMatrix<3>,
    pixels: Vec<Vec<Pixel>>,
}

impl RGBFilm {
    pub fn new(
        _resolution: Point2i,
        _filename: &String,
        sensor: Arc<PixelSensor>,
        filter: Arc<dyn Filter>,
        global_variable: &GlobalVariable,
    ) -> Self {
        let width = _resolution.x;
        let height = _resolution.y;

        let postfix = get_postfix(_filename);
        let png_filename = if postfix == "png" {
            _filename.clone()
        } else {
            change_postfix(_filename, "png")
        };

        let output_rgb_from_sensor_rgb =
            global_variable.srgb_color_space.rgb_from_xyz * sensor.xyz_from_sensor_rgb;

        return RGBFilm {
            resolution: _resolution.clone(),
            filename: png_filename,
            sensor,
            filter: filter.clone(),
            output_rgb_from_sensor_rgb,
            pixels: vec![vec![Pixel::default(); width as usize]; height as usize],
        };
    }
}

impl Film for RGBFilm {
    fn get_resolution(&self) -> Point2i {
        return self.resolution;
    }

    fn get_filter(&self) -> Arc<dyn Filter> {
        return self.filter.clone();
    }

    fn get_pixel_rgb(&self, p: Point2i) -> RGB {
        let pixel = self.pixels[p.x as usize][p.y as usize];
        let raw_rgb = RGB::new(
            pixel.rgb_sum[0] as Float,
            pixel.rgb_sum[1] as Float,
            pixel.rgb_sum[2] as Float,
        );

        // Normalize _rgb_ with weight sum
        let rgb = if pixel.weight_sum != 0.0 {
            raw_rgb / (pixel.weight_sum as Float)
        } else {
            raw_rgb
        };

        // Add splat value at pixel
        // TODO: not implemented: Add splat value at pixel

        return self.output_rgb_from_sensor_rgb * rgb;
    }

    fn sample_wavelengths(&self, u: Float) -> SampledWavelengths {
        return SampledWavelengths::sample_visible(u);
    }

    fn add_sample(
        &mut self,
        point_film: Point2i,
        l: &SampledSpectrum,
        lambda: &SampledWavelengths,
        weight: Float,
    ) {
        // Convert sample radiance to _PixelSensor_ RGB
        let rgb = self.sensor.to_sensor_rgb(l, lambda);

        /*
        // TODO: clamp m like PBRT-v4
        // Optionally clamp sensor RGB value
        Float m = std::max({rgb.r, rgb.g, rgb.b});
        if (m > maxComponentValue)
            rgb *= maxComponentValue / m;
        */

        // Update pixel values with filtered sample contribution
        let pixel = &mut self.pixels[point_film.x as usize][point_film.y as usize];
        for c in 0..3 {
            pixel.rgb_sum[c] += (weight * rgb[c]) as f64;
        }
        pixel.weight_sum += weight as f64;
    }

    fn export_image(&self) {
        let mut buffer: RgbImage =
            ImageBuffer::new(self.resolution.x as u32, self.resolution.y as u32);

        for (x, y, mut_pixel) in buffer.enumerate_pixels_mut() {
            let rgb = self.get_pixel_rgb(Point2i::new(x as i32, y as i32));

            let gamma_rgb = RGB::new(rgb.r.sqrt(), rgb.g.sqrt(), rgb.b.sqrt());
            let u16_rgb = gamma_rgb * (256.0 - 0.0001);

            *mut_pixel = Rgb([u16_rgb.r as u8, u16_rgb.g as u8, u16_rgb.b as u8]);
        }

        buffer.save(self.filename.clone()).unwrap();
        println!("image saved to `{}`", self.filename.clone());
    }
}
