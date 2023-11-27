use crate::pbrt::*;
use image::{ImageBuffer, Rgb, RgbImage};

#[derive(Clone, Copy)]
pub enum WrapMode {
    Black,
    Clamp,
    Repeat,
    OctahedralSphere,
}

#[derive(Clone, Copy)]
pub enum PixelFormat {
    U256,
    Half,
    Float,
}

#[derive(Clone)]
pub struct ResampleWeight {
    pub first_pixel: usize,
    pub weight: [Float; 4],
}

impl Default for ResampleWeight {
    fn default() -> Self {
        return Self {
            first_pixel: usize::MAX,
            weight: [Float::NAN; 4],
        };
    }
}

pub fn parse_wrap_mode(wrap_string: &str) -> WrapMode {
    return match wrap_string {
        "black" => WrapMode::Black,
        "clamp" => WrapMode::Clamp,
        "repeat" => WrapMode::Repeat,
        "octahedralsphere" => WrapMode::OctahedralSphere,
        &_ => {
            panic!("unknown wrap mode: `{}`", wrap_string);
        }
    };
}

#[derive(Clone)]
pub struct Image {
    pub resolution: Point2i,
    pixels: Vec<Vec<RGB>>,
    pub pixel_format: PixelFormat,
}

fn resample_weights(old_resolution: usize, new_resolution: usize) -> Vec<ResampleWeight> {
    assert!(new_resolution >= old_resolution);
    let mut wt = vec![ResampleWeight::default(); new_resolution];
    let filter_radius = 2.0;
    let tau = 2.0;

    for i in 0..new_resolution {
        // Compute image resampling weights for _i_th pixel
        let center = (i as Float + 0.5) * (old_resolution as Float) / (new_resolution as Float);
        wt[i].first_pixel = ((center - filter_radius) + 0.5).floor() as usize;
        for j in 0..4 {
            let pos = (wt[i].first_pixel + j) as Float + 0.5;
            wt[i].weight[j] = windowed_sinc(pos - center, filter_radius, tau)
        }

        // Normalize filter weights for pixel resampling
        let inv_sum_weights = 1.0 / (wt[i].weight.into_par_iter().sum::<Float>());
        wt[i].weight = wt[i].weight.map(|x| x * inv_sum_weights);
    }

    return wt;
}

pub fn generate_pyramid(image: Image, wrap_mode: WrapMode) -> Vec<Image> {
    // TODO: generate_pyramid: to verify
    let image = if !is_power_of_2(image.resolution.x) || !is_power_of_2(image.resolution.y) {
        image.float_resize_up(
            Point2i::new(
                round_up_pow_2(image.resolution.x),
                round_up_pow_2(image.resolution.y),
            ),
            wrap_mode,
        )
    } else {
        image
    };

    // Initialize levels of pyramid from _image_
    let n_levels = 1 + (image.resolution.x.max(image.resolution.y)).ilog2();
    let mut pyramid = Vec::<Image>::new();
    pyramid.reserve(n_levels as usize);
    pyramid.push(image.clone());

    let pixel_format = image.pixel_format;

    for i in 0..n_levels - 1 {
        // TODO: this part is different than PBRT-v4

        let resolution = if i == 0 {
            image.resolution
        } else {
            pyramid[(i - 1) as usize].resolution
        };

        // Initialize $i+1$st level from $i$th level and copy $i$th into pyramid
        // Initialize _nextImage_ for $i+1$st level
        let next_resolution = Point2i::new(1.max(resolution.x / 2), 1.max(resolution.y / 2));

        let mut next_image = Image::new(next_resolution, pixel_format);
        for y in 0..(next_resolution.y as usize) {
            for x in 0..(next_resolution.x as usize) {
                next_image[y][x] = (image[y][x]
                    + image[y * 2][x * 2]
                    + image[y * 2 + 1][x * 2]
                    + image[y * 2][x * 2 + 1])
                    / 4.0;
            }
        }

        pyramid.push(next_image);
    }

    return pyramid;
}

impl Image {
    pub fn new(resolution: Point2i, pixel_format: PixelFormat) -> Self {
        assert!(resolution.x > 0);
        assert!(resolution.y > 0);

        return Self {
            resolution,
            pixels: vec![vec![RGB::black(); resolution.x as usize]; resolution.y as usize],
            // TODO: swap x and y to make it aligned with RgbImage dimension
            pixel_format,
        };
    }

    pub fn read_from_file(filename: &str) -> Self {
        if get_extension(filename) != "png" {
            panic!("only PNG file is supported for the moment");
        }

        let img = match image::open(filename) {
            Ok(_dynamic_img) => _dynamic_img.into_rgb8(),
            Err(_) => {
                panic!("fail to read `{}`", filename)
            }
        };

        let (width, height) = img.dimensions();

        const DIVISOR: Float = u8::MAX as Float;

        let mut pixels = vec![vec![RGB::black(); width as usize]; height as usize];
        for x in 0..width {
            for y in 0..height {
                let rgb_u256 = img[(x, y)].0;

                // TODO: check if this is reversible with export_png()
                pixels[y as usize][x as usize] = RGB::new(
                    rgb_u256[0] as Float / DIVISOR,
                    rgb_u256[1] as Float / DIVISOR,
                    rgb_u256[2] as Float / DIVISOR,
                );
            }
        }

        return Self {
            resolution: Point2::new(width as i32, height as i32),
            pixels,
            pixel_format: PixelFormat::U256,
        };
    }

    pub fn float_resize_up(&self, new_resolution: Point2i, wrap_mode: WrapMode) -> Image {
        //TODO: ignore float_resize_up() for the moment

        panic!("Image::float_resize_up() is not implemented");

        assert!(new_resolution.x >= self.resolution.x);
        assert!(new_resolution.y >= self.resolution.y);

        let resampled_image = Image::new(new_resolution, PixelFormat::Float);

        let x_weights = resample_weights(self.resolution.x as usize, new_resolution.x as usize);

        let y_weights = resample_weights(self.resolution.y as usize, new_resolution.y as usize);

        unreachable!();
    }

    pub fn export_to_png(&self, filename: &str) {
        let mut buffer: RgbImage =
            ImageBuffer::new(self.resolution.x as u32, self.resolution.y as u32);

        for y in 0..self.resolution.y {
            for x in 0..self.resolution.x {
                let u256 = self.pixels[y as usize][x as usize]
                    .gamma_correction()
                    .to_u256();

                buffer.put_pixel(x as u32, y as u32, Rgb(u256));
            }
        }

        buffer.save(filename).unwrap();
    }
}

impl Index<usize> for Image {
    type Output = Vec<RGB>;

    fn index(&self, index: usize) -> &Self::Output {
        return &self.pixels[index];
    }
}

impl IndexMut<usize> for Image {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        return &mut self.pixels[index];
    }
}
