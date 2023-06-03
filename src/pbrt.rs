pub type Float = f32;
// pub type Float = f64;

pub enum RenderingCoordinateSystem {
    Camera,
    CameraWorld,
    World,
}

pub use fma::fma;
pub use serde_json::Value;
pub use std::collections::HashMap;
pub use std::fs::File;
pub use std::io::Read;
pub use std::mem;
pub use std::mem::swap;
pub use std::ops::{Add, Div, Index, IndexMut, Mul, Sub};

pub use crate::cameras::*;
pub use crate::math::arithmatic::*;
pub use crate::math::point::*;
pub use crate::math::square_matrix::*;
pub use crate::math::vector::*;
pub use crate::scene_parser::parameter_dict::*;
pub use crate::scene_parser::scene_builder::*;
pub use crate::scene_parser::util::*;
pub use crate::transform::*;

pub type Point2f = Point2<Float>;
