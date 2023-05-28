pub type Float = f32;
// pub type Float = f64;

pub use fma::fma;
pub use serde_json::Value;
pub use std::fs::File;
pub use std::io::Read;
pub use std::mem;
pub use std::ops;
pub use std::ops::{Add, Div, Mul, Sub};

pub use crate::euclidean_geometry::math::*;
pub use crate::euclidean_geometry::point::*;
pub use crate::euclidean_geometry::square_matrix::*;
pub use crate::euclidean_geometry::vector::*;

pub use crate::scene_builder::*;
pub use crate::transform::*;

pub type Point2f = Point2<Float>;
