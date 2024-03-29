lazy_static! {
    pub static ref COLOR_SPACE: RGBColorSpace = {
        let srgb_to_spectrum_table = RGBtoSpectrumTable::new("sRGB");

        RGBColorSpace::new(
            Point2f::new(0.64, 0.33),
            Point2f::new(0.3, 0.6),
            Point2f::new(0.15, 0.06),
            get_named_spectrum("stdillum-D65"),
            srgb_to_spectrum_table,
        )
    };
    pub static ref COLOR_ENCODING: SRGBColorEncoding = SRGBColorEncoding {};
}

pub const ONE_MINUS_EPSILON: f64 = {
    let bits_of_one = 1.0f64.to_bits();
    f64::from_bits(bits_of_one - 1)
};

pub use fma::fma;
pub use lazy_static::lazy_static;
pub use rand::{rngs::StdRng, thread_rng, Rng, SeedableRng};
pub use rand_distr::num_traits::Float;
pub use rayon::prelude::*;
pub use std::{
    any::{type_name, Any, TypeId},
    cmp::Ordering,
    collections::{hash_map::DefaultHasher, HashMap, HashSet},
    fmt::{Debug, Display, Formatter},
    fs::{read_to_string, File},
    hash::{Hash, Hasher},
    io::{BufReader, Read},
    iter::Sum,
    ops::{
        Add, AddAssign, BitAnd, BitOr, BitOrAssign, Div, Index, IndexMut, Mul, MulAssign, Neg, Sub,
    },
    process::exit,
    str::FromStr,
    sync::{
        atomic::AtomicUsize,
        {Arc, Mutex},
    },
    time::Instant,
};
pub use strum::IntoEnumIterator;
pub use strum_macros::EnumIter;

pub use crate::base::bsdf::*;
pub use crate::{
    accelerator::bvh::*,
    base::{
        bxdf::*, camera::*, film::*, filter::*, integrator::*, interaction::*, light::*,
        light_sampler::*, material::*, medium::*, primitive::*, ray::*, sampler::*, shape::*,
        spectrum::*, texture::*,
    },
    bxdfs::{coated_diffuse::*, dielectric::*, diffuse::*, layered::*, top_or_bottom::*},
    cameras::perspective::*,
    euclidean_space::{
        bounds::*, compensated_float::*, float::*, frame::*, interval::*, interval::*, normal::*,
        point2::*, point3::*, square_matrix::*, transform::*, vector2::*, vector3::*,
    },
    films::{pixel_sensor::*, rgb_film::*},
    filters::box_filter::*,
    integrators::{ambient_occlusion::*, random_walk::*, simple_path::*, surface_normal::*},
    light_samplers::uniform_light_sampler::*,
    lights::{diffuse_area::*, distant::*},
    materials::{coated_diffuse::*, diffuse::*},
    primitives::{geometric_primitive::*, simple_primitive::*},
    samplers::independent::*,
    scene::{lexer::*, parameter_dict::*, renderer::*, scene_builder::*, util::*},
    shapes::{loop_subdivision::*, sphere::*, tri_quad_mesh::*, triangle::*, triangle_mesh::*},
    spectra::{
        black_body_spectrum::*, cie_xyz::*, const_piecewise_linear_spectrum::*, const_spectrum::*,
        densely_sampled_spectrum::*, measured_spectra_data::*, piecewise_linear_spectrum::*,
        rgb::*, rgb_albedo_spectrum::*, rgb_illuminant_spectrum::*, rgb_sigmoid_polynomial::*,
        rgb_to_spectrum_data::*, rgb_to_spectrum_table::*, sampled_spectrum::*,
        sampled_wavelengths::*,
    },
    textures::{
        float_constant_texture::*, mipmap::*, spectrum_constant_texture::*,
        spectrum_image_texture::*, spectrum_scaled_texture::*, texture_mapping_2d::*,
        uv_mapping::*,
    },
    util::{
        color::*, color_encoding::*, colorspace::*, image::*, math::*, sampling::*, scattering::*,
    },
};

pub type Point2f = Point2<f64>;
pub type Point2i = Point2<i32>;
pub type Point3f = Point3<f64>;
pub type Point3fi = Point3<Interval>;

pub type Vector2f = Vector2<f64>;
pub type Vector3f = Vector3<f64>;
pub type Vector3fi = Vector3<Interval>;

pub type Bounds2i = Bounds2<i32>;
pub type Bounds2f = Bounds2<f64>;
pub type Bounds3f = Bounds3<f64>;
