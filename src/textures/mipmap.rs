use crate::pbrt::*;

#[derive(Clone, Copy, PartialEq, PartialOrd)]
pub enum FilterFunction {
    Point,
    Bilinear,
    Trilinear,
    EWA,
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
    pyramid: Vec<Image>,
    color_space: Arc<RGBColorSpace>,
    wrap_mode: WrapMode,
    options: MIPMapFilterOptions,
}

impl MIPMap {
    fn MIPMap(
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

    pub fn create_from_file(
        filename: &str,
        options: MIPMapFilterOptions,
        wrap_mode: WrapMode,
        global_variable: &GlobalVariable,
    ) -> Self {
        let image = Image::read_from_file(filename);
        return MIPMap::MIPMap(
            image,
            global_variable.rgb_color_space.clone(),
            wrap_mode,
            options,
        );
    }
}
