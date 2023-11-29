use crate::pbrt::*;

#[derive(Clone, Copy, PartialEq, PartialOrd)]
pub enum FilterFunction {
    Point,
    Bilinear,
    Trilinear,
    EWA,
}

impl Display for FilterFunction {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "[ FilterFunction: {} ]",
            match *self {
                FilterFunction::Point => {
                    "Point"
                }
                FilterFunction::Bilinear => {
                    "Bilnear"
                }
                FilterFunction::Trilinear => {
                    "Trilinear"
                }
                FilterFunction::EWA => {
                    "EWA"
                }
            }
        )
    }
}

pub fn parse_filter_function(filter_function: &str) -> FilterFunction {
    return match filter_function {
        "ewa" => FilterFunction::EWA,
        "bilinear" => FilterFunction::Bilinear,
        "trilinear" => FilterFunction::Trilinear,
        "point" => FilterFunction::Point,
        _ => {
            panic!("unknown filter function `{}`", filter_function);
        }
    };
}

#[derive(Clone, Copy)]
pub struct MIPMapFilterOptions {
    pub filter: FilterFunction,
    pub max_anisotropy: Float,
}

impl Display for MIPMapFilterOptions {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "MIPMapFilterOptions [ filter: {}, max_anisotropy: {} ]",
            self.filter, self.max_anisotropy
        )
    }
}

impl Default for MIPMapFilterOptions {
    fn default() -> Self {
        return Self {
            filter: FilterFunction::EWA,
            max_anisotropy: 8.0,
        };
    }
}

impl PartialEq for MIPMapFilterOptions {
    fn eq(&self, other: &Self) -> bool {
        return self.filter == other.filter && self.max_anisotropy == other.max_anisotropy;
    }
}

impl PartialOrd for MIPMapFilterOptions {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        return match self.filter.partial_cmp(&other.filter) {
            None => None,
            Some(ordering) => match ordering {
                Ordering::Less => Some(Ordering::Less),
                Ordering::Greater => Some(Ordering::Greater),
                Ordering::Equal => self.max_anisotropy.partial_cmp(&other.max_anisotropy),
            },
        };
    }
}

pub struct MIPMap {
    pub pyramid: Vec<Image>,
    pub color_space: Arc<RGBColorSpace>,
    pub wrap_mode: WrapMode,
    pub options: MIPMapFilterOptions,
}

impl MIPMap {
    fn new(
        image: Image,
        color_space: Arc<RGBColorSpace>,
        wrap_mode: WrapMode,
        options: MIPMapFilterOptions,
    ) -> Self {
        return Self {
            pyramid: generate_pyramid(image, wrap_mode),
            color_space,
            wrap_mode,
            options,
        };
    }

    fn levels(&self) -> usize {
        return self.pyramid.len();
    }

    fn texel(&self, level: usize, st: Point2i) -> RGB {
        return self.pyramid[level][st.y as usize][st.x as usize];
    }

    fn bilerp(&self, level: usize, st: Point2f) -> RGB {
        return self.pyramid[level].bilerp(st, WrapMode2D::new([self.wrap_mode, self.wrap_mode]));
    }

    pub fn create_from_file(
        filename: &str,
        options: MIPMapFilterOptions,
        wrap_mode: WrapMode,
        global_variable: &GlobalVariable,
    ) -> Self {
        let image = Image::read_from_file(filename);
        return MIPMap::new(
            image,
            global_variable.rgb_color_space.clone(),
            wrap_mode,
            options,
        );
    }

    pub fn filter(&self, st: Point2f, dst0: Vector2f, dst1: Vector2f) -> RGB {
        if self.options.filter != FilterFunction::EWA {
            // Handle non-EWA MIP Map filter
            let width = 2.0
                * (dst0[0]
                    .abs()
                    .max(dst0[1].abs())
                    .max(dst1[0].abs())
                    .max(dst1[1].abs()));

            // Compute MIP Map level for _width_ and handle very wide filter
            let n_levels = self.levels();
            let level = (n_levels - 1) as Float + (width as Float).max(1e-8).log2();

            if level >= (n_levels - 1) as Float {
                return self.texel(n_levels - 1, Point2i::new(0, 0));
            }

            let i_level = (level.floor() as usize).max(0);

            match self.options.filter {
                FilterFunction::Bilinear => {
                    return self.bilerp(i_level, st);
                }

                _ => {
                    panic!("`{}` not implemented", self.options.filter);
                }
            };
        }

        panic!("`{}` not implemented", self.options.filter);
    }
}
