#[cfg(feature = "use_f64")]
pub type Float = f64;

#[cfg(not(feature = "use_f64"))]
pub type Float = f32;

pub struct GlobalVariable {
    pub rgb_color_space: Arc<RGBColorSpace>,
}

pub use fma::fma;
pub use rand::{rngs::StdRng, thread_rng, Rng, SeedableRng};
pub use rayon::prelude::*;
pub use serde_json::Value;
pub use std::{
    any::{type_name, Any, TypeId},
    cmp::Ordering,
    collections::{HashMap, HashSet},
    fmt::{Debug, Display, Formatter},
    fs::File,
    hash::{Hash, Hasher},
    io::{BufReader, Read},
    iter::Sum,
    ops::{Add, AddAssign, BitAnd, BitOr, Div, Index, IndexMut, Mul, MulAssign, Neg, Sub},
    process::exit,
    str::FromStr,
    sync::{
        atomic::AtomicUsize,
        {Arc, Mutex},
    },
    time::Instant,
};

pub use crate::{
    accelerator::bvh::*,
    base::{
        bxdf::*, camera::*, film::*, filter::*, integrator::*, primitive::*, ray::*, sampler::*,
        shape::*, spectrum::*, texture::*,
    },
    bxdfs::diffuse_bxdf::*,
    cameras::perspective::*,
    euclidean_space::{
        bounds::*, compensated_float::*, float::*, frame::*, interval::*, interval::*, normal::*,
        point2::*, point3::*, square_matrix::*, transform::*, vector2::*, vector3::*,
    },
    films::{pixel_sensor::*, rgb_film::*},
    filters::box_filter::*,
    integrators::{ambient_occlusion::*, surface_normal::*},
    primitives::simple_primitive::*,
    samplers::independent::*,
    scene::{parameter_dict::*, renderer::*, scene_builder::*, util::*},
    shapes::{loop_subdivision::*, sphere::*, tri_quad_mesh::*, triangle::*, triangle_mesh::*},
    spectra::{
        black_body_spectrum::*, cie_xyz::*, const_piecewise_linear_spectrum::*,
        densely_sampled_spectrum::*, measured_spectra_data::*, piecewise_linear_spectrum::*,
        rgb::*, rgb_sigmoid_polynomial::*, rgb_to_spectrum_data::*, rgb_to_spectrum_table::*,
        sampled_spectrum::*, sampled_wavelengths::*,
    },
    textures::{
        float_constant_texture::*, mipmap::*, spectrum_constant_texture::*,
        spectrum_image_texture::*, spectrum_scaled_texture::*, texture_mapping_2d::*,
        uv_mapping::*,
    },
    util::{color::*, color_encoding::*, colorspace::*, image::*, math::*, sampling::*},
};

pub type Point2f = Point2<Float>;
pub type Point2i = Point2<i32>;
pub type Point3f = Point3<Float>;
pub type Point3fi = Point3<Interval>;

pub type Vector2f = Vector2<Float>;
pub type Vector3f = Vector3<Float>;
pub type Vector3fi = Vector3<Interval>;

pub type Bounds2f = Bounds2<Float>;
pub type Bounds3f = Bounds3<Float>;

pub fn same_type<T0: ?Sized + Any, T1: ?Sized + Any>() -> bool {
    return TypeId::of::<T0>() == TypeId::of::<T1>();
}
