use crate::pbrt::*;

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
    resolution: Point2i,
    filename: String,
    filter: Arc<dyn Filter>,
    sensor: Arc<PixelSensor>,
    output_rgb_from_sensor_rgb: SquareMatrix<3>,
    pixels: Vec<Vec<Pixel>>,
}

impl RGBFilm {
    pub fn new(
        resolution: Point2i,
        filename: &String,
        sensor: Arc<PixelSensor>,
        filter: Arc<dyn Filter>,
        global_variable: &GlobalVariable,
    ) -> Self {
        let width = resolution.x;
        let height = resolution.y;

        let extension = get_extension(filename);
        let png_filename = if extension == "png" {
            filename.clone()
        } else {
            change_extension(filename, "png")
        };

        let output_rgb_from_sensor_rgb =
            global_variable.rgb_color_space.rgb_from_xyz * sensor.xyz_from_sensor_rgb;

        return RGBFilm {
            resolution: resolution.clone(),
            filename: png_filename,
            sensor,
            filter: filter.clone(),
            output_rgb_from_sensor_rgb,
            pixels: vec![vec![Pixel::default(); width as usize]; height as usize],
        };
    }
}

impl Film for RGBFilm {
    fn fork(&self) -> Box<dyn Film> {
        return Box::new(RGBFilm {
            resolution: self.resolution,
            filename: self.filename.clone(),
            filter: self.filter.clone(),
            sensor: self.sensor.clone(),
            output_rgb_from_sensor_rgb: self.output_rgb_from_sensor_rgb,
            pixels: vec![
                vec![Pixel::default(); self.resolution.x as usize];
                self.resolution.y as usize
            ],
        });
    }

    fn convert_to_rgb_film(&self) -> RGBFilm {
        return self.clone();
    }

    fn get_filename(&self) -> String {
        return self.filename.clone();
    }

    fn get_resolution(&self) -> Point2i {
        return self.resolution;
    }

    fn get_filter(&self) -> Arc<dyn Filter> {
        return self.filter.clone();
    }

    fn get_pixel_rgb(&self, p: Point2i) -> RGB {
        let pixel = self.pixels[p.y as usize][p.x as usize];
        let raw_rgb = RGB::new(
            pixel.rgb_sum[0] as f64,
            pixel.rgb_sum[1] as f64,
            pixel.rgb_sum[2] as f64,
        );

        // Normalize _rgb_ with weight sum
        let rgb = if pixel.weight_sum != 0.0 {
            raw_rgb / (pixel.weight_sum as f64)
        } else {
            raw_rgb
        };

        // Add splat value at pixel
        // TODO: not implemented: Add splat value at pixel

        return self.output_rgb_from_sensor_rgb * rgb;
    }

    fn merge(&mut self, film: &dyn Film, y_list: Vec<usize>) {
        let rgb_film = film.convert_to_rgb_film();
        assert_eq!(self.resolution, rgb_film.get_resolution());

        for y in y_list {
            self.pixels[y] = rgb_film.pixels[y].clone();
        }
    }

    fn add_sample(
        &mut self,
        point_film: Point2i,
        l: &SampledSpectrum,
        lambda: &SampledWavelengths,
        weight: f64,
    ) {
        // Convert sample radiance to _PixelSensor_ RGB
        let rgb = self.sensor.to_sensor_rgb(l, lambda);

        /*
        // TODO: clamp m like PBRT-v4
        // Optionally clamp sensor RGB value
        f64 m = std::max({rgb.r, rgb.g, rgb.b});
        if (m > maxComponentValue)
            rgb *= maxComponentValue / m;
        */

        // Update pixel values with filtered sample contribution
        let pixel = &mut self.pixels[point_film.y as usize][point_film.x as usize];
        for c in 0..3 {
            pixel.rgb_sum[c] += (weight * rgb[c]) as f64;
        }
        pixel.weight_sum += weight as f64;
    }
}
