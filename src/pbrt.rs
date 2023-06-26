pub type Float = f32;
// pub type Float = f64;

pub const X_RESOLUTION: i32 = 1368;
pub const Y_RESOLUTION: i32 = 1026;

pub enum RenderingCoordinateSystem {
    Camera,
    CameraWorld,
    World,
}

pub use fma::fma;
pub use rand::rngs::StdRng;
pub use rand::{thread_rng, Rng, SeedableRng};
pub use serde_json::Value;
pub use std::collections::HashMap;
pub use std::fmt::Debug;
pub use std::fs::File;
pub use std::io::Read;
pub use std::mem;
pub use std::mem::swap;
pub use std::ops::{Add, Div, Index, IndexMut, Mul, Sub};
pub use std::process::exit;
pub use std::str::FromStr;
pub use std::sync::Arc;
pub use std::sync::Mutex;

pub use crate::base::cameras::*;
pub use crate::base::film::*;
pub use crate::base::filter::*;
pub use crate::base::integrator::*;
pub use crate::base::sampler::*;
pub use crate::math::arithmetic::*;
pub use crate::math::bounds::*;
pub use crate::math::compensated_float::*;
pub use crate::math::point::*;
pub use crate::math::square_matrix::*;
pub use crate::math::transform::*;
pub use crate::math::vector::*;
pub use crate::scene_parser::parameter_dict::*;
pub use crate::scene_parser::scene_builder::*;
pub use crate::scene_parser::util::*;
pub use crate::shapes::triangle::*;
