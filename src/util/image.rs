use crate::pbrt::*;
use image::{ImageBuffer, Rgb, RgbImage};

#[derive(Clone, Copy, PartialEq)]
pub enum WrapMode {
    Black,
    Clamp,
    Repeat,
    OctahedralSphere,
}

impl Display for WrapMode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match *self {
                WrapMode::Black => {
                    "Black"
                }
                WrapMode::Clamp => {
                    "Clamp"
                }
                WrapMode::Repeat => {
                    "Repeat"
                }
                WrapMode::OctahedralSphere => {
                    "OctahedralSphere"
                }
            }
        )
    }
}

#[derive(Clone, Copy)]
pub struct WrapMode2D {
    pub wrap: [WrapMode; 2],
}

impl Index<usize> for WrapMode2D {
    type Output = WrapMode;

    fn index(&self, index: usize) -> &Self::Output {
        return &self.wrap[index];
    }
}

impl WrapMode2D {
    pub fn new(wrap: [WrapMode; 2]) -> Self {
        return WrapMode2D { wrap };
    }
}

#[derive(Clone, Copy)]
pub enum PixelFormat {
    U256,
    Half,
    f64,
}

#[derive(Clone)]
pub struct ResampleWeight {
    pub first_pixel: usize,
    pub weight: [f64; 4],
}

impl Default for ResampleWeight {
    fn default() -> Self {
        return Self {
            first_pixel: usize::MAX,
            weight: [f64::NAN; 4],
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
        let center = (i as f64 + 0.5) * (old_resolution as f64) / (new_resolution as f64);
        wt[i].first_pixel = ((center - filter_radius) + 0.5).floor() as usize;
        for j in 0..4 {
            let pos = (wt[i].first_pixel + j) as f64 + 0.5;
            wt[i].weight[j] = windowed_sinc(pos - center, filter_radius, tau)
        }

        // Normalize filter weights for pixel resampling
        let inv_sum_weights = 1.0 / (wt[i].weight.into_par_iter().sum::<f64>());
        wt[i].weight = wt[i].weight.map(|x| x * inv_sum_weights);
    }

    return wt;
}

pub fn generate_pyramid(image: Image, wrap_mode: WrapMode) -> Vec<Image> {
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

    let pixel_format = image.pixel_format;

    let mut pyramid = Vec::<Image>::new();
    pyramid.reserve(n_levels as usize);
    pyramid.push(image);

    for i in 1..n_levels {
        // TODO: this part is different than PBRT-v4

        let last_image = &pyramid[(i - 1) as usize];
        let resolution = last_image.resolution;

        // Initialize $i+1$st level from $i$th level and copy $i$th into pyramid
        // Initialize _nextImage_ for $i+1$st level
        let next_resolution = Point2i::new(1.max(resolution.x / 2), 1.max(resolution.y / 2));

        let mut next_image = Image::new(next_resolution, pixel_format);
        for y in 0..(next_resolution.y as usize) {
            for x in 0..(next_resolution.x as usize) {
                next_image[y][x] = (last_image[y * 2][x * 2]
                    + last_image[y * 2][x * 2 + 1]
                    + last_image[y * 2 + 1][x * 2]
                    + last_image[y * 2 + 1][x * 2 + 1])
                    / 4.0;
            }
        }

        pyramid.push(next_image);
    }

    return pyramid;
}

fn remap_pixel_coord(p: Point2i, resolution: Point2i, wrap_mode2d: WrapMode2D) -> Point2i {
    if wrap_mode2d[0] == WrapMode::OctahedralSphere || wrap_mode2d[1] == WrapMode::OctahedralSphere
    {
        panic!("WrapMode::OctahedralSphere not implemented");
    }

    let mut coord = p;

    for c in 0..2 {
        if coord[c] >= 0 && coord[c] < resolution[c] {
            continue;
        }
        match wrap_mode2d[c] {
            WrapMode::Repeat => {
                coord[c] = mod_i32(coord[c], resolution[c]);
            }
            _ => {
                panic!("`{}` not implemented", wrap_mode2d[c]);
            }
        }
    }

    return coord;
}

impl Image {
    pub fn new(resolution: Point2i, pixel_format: PixelFormat) -> Self {
        assert!(resolution.x > 0);
        assert!(resolution.y > 0);

        return Self {
            resolution,
            pixels: vec![vec![RGB::black(); resolution.x as usize]; resolution.y as usize],
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
        let mut pixels = vec![vec![RGB::black(); width as usize]; height as usize];
        for x in 0..width {
            for y in 0..height {
                let rgb_u256 = img[(x, y)].0;

                pixels[y as usize][x as usize] = RGB::new(
                    COLOR_ENCODING.to_linear(rgb_u256[0]),
                    COLOR_ENCODING.to_linear(rgb_u256[1]),
                    COLOR_ENCODING.to_linear(rgb_u256[2]),
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
        assert!(new_resolution.x >= self.resolution.x);
        assert!(new_resolution.y >= self.resolution.y);

        let mut resampled_image = Image::new(new_resolution, self.pixel_format);

        for y in 0..(new_resolution.y as usize) {
            for x in 0..(new_resolution.x as usize) {
                let low_res_y = (y as f64) / (new_resolution.y as f64) * (self.resolution.y as f64);
                let low_res_x = (x as f64) / (new_resolution.x as f64) * (self.resolution.x as f64);
                resampled_image[y][x] = self[low_res_y as usize][low_res_x as usize];
            }
        }

        return resampled_image;

        // TODO: progress 2023/12/29 implementing float_resize_up()

        unreachable!();

        let resampled_image = Image::new(new_resolution, PixelFormat::f64);

        let x_weights = resample_weights(self.resolution.x as usize, new_resolution.x as usize);

        let y_weights = resample_weights(self.resolution.y as usize, new_resolution.y as usize);

        let out_extent = Bounds2i::new(&[Point2i::new(0, 0), new_resolution]);

        let in_extent = {
            let _in_extent_min = Point2i::new(
                x_weights[out_extent.p_min.x as usize].first_pixel as i32,
                y_weights[out_extent.p_min.y as usize].first_pixel as i32,
            );

            let _in_extent_max = Point2i::new(
                x_weights[out_extent.p_max.x as usize - 1].first_pixel as i32 + 4,
                y_weights[out_extent.p_max.y as usize - 1].first_pixel as i32 + 4,
            );

            Bounds2i::new(&[_in_extent_min, _in_extent_max])
        };

        /*
        std::vector<float> inBuf(NChannels() * inExtent.Area());
        CopyRectOut(inExtent, pstd::span<float>(inBuf), wrapMode);
        */

        panic!("Image::float_resize_up() is not implemented");
    }

    pub fn export_to_png(&self, filename: &str) {
        let mut buffer: RgbImage =
            ImageBuffer::new(self.resolution.x as u32, self.resolution.y as u32);

        for y in 0..self.resolution.y {
            for x in 0..self.resolution.x {
                let color = self.pixels[y as usize][x as usize];

                let red = COLOR_ENCODING.to_srgb8(color.r);
                let green = COLOR_ENCODING.to_srgb8(color.g);
                let blue = COLOR_ENCODING.to_srgb8(color.b);

                buffer.put_pixel(x as u32, y as u32, Rgb([red, green, blue]));
            }
        }

        buffer.save(filename).unwrap();
    }

    fn fetch_pixel(&self, p: Point2i, wrap_mode2d: WrapMode2D) -> RGB {
        let p = remap_pixel_coord(p, self.resolution, wrap_mode2d);
        return self.pixels[p.y as usize][p.x as usize];
    }

    pub fn bilerp(&self, p: Point2f, wrap_mode2d: WrapMode2D) -> RGB {
        // Compute discrete pixel coordinates and offsets for _p_
        let x = p[0] * (self.resolution.x as f64) - 0.5;
        let y = p[1] * (self.resolution.y as f64) - 0.5;

        let xi = x.floor() as i32;
        let yi = y.floor() as i32;

        let dx = x - (xi as f64);
        let dy = y - (yi as f64);

        // Load pixel channel values and return bilinearly interpolated value
        let v = [
            self.fetch_pixel(Point2i::new(xi, yi), wrap_mode2d),
            self.fetch_pixel(Point2i::new(xi + 1, yi), wrap_mode2d),
            self.fetch_pixel(Point2i::new(xi, yi + 1), wrap_mode2d),
            self.fetch_pixel(Point2i::new(xi + 1, yi + 1), wrap_mode2d),
        ];

        return (1.0 - dx) * (1.0 - dy) * v[0]
            + dx * (1.0 - dy) * v[1]
            + (1.0 - dx) * dy * v[2]
            + dx * dy * v[3];
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
