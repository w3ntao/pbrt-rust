pub type Float = f32;
//pub type Float = f64;

pub enum RenderingCoordinateSystem {
    Camera,
    CameraWorld,
    World,
}

pub use fma::fma;
pub use rand::{rngs::StdRng, thread_rng, Rng, SeedableRng};
pub use serde_json::Value;
pub use std::collections::HashMap;
pub use std::fmt::{Debug, Display, Formatter};
pub use std::fs::File;
pub use std::io::Read;
pub use std::mem::swap;
pub use std::ops::{Add, Div, Index, IndexMut, Mul, Neg, Sub};
pub use std::process::exit;
pub use std::str::FromStr;
pub use std::sync::{Arc, Mutex};

pub use crate::base::camera::*;
pub use crate::base::film::*;
pub use crate::base::filter::*;
pub use crate::base::integrator::*;
pub use crate::base::ray::*;
pub use crate::base::rgb_color::*;
pub use crate::base::sampler::*;
pub use crate::base::shape::*;
pub use crate::base::shape_intersection::*;
pub use crate::cameras::perspective_camera::*;
pub use crate::math::arithmetic::*;
pub use crate::math::compensated_float::*;
pub use crate::math::float::*;
pub use crate::math::interval::*;
pub use crate::math::square_matrix::*;
pub use crate::math::transform::*;
pub use crate::scene::parameter_dict::*;
pub use crate::scene::scene_builder::*;
pub use crate::scene::scene_config::*;
pub use crate::scene::util::*;
pub use crate::shapes::triangle::*;

use crate::math::bounds::*;
use crate::math::interval::*;
use crate::math::point::*;
use crate::math::vector::*;

pub type Point2f = Point2<Float>;
pub type Point2i = Point2<i32>;
pub type Point3f = Point3<Float>;
pub type Point3fi = Point3<Interval>;

pub type Vector2f = Vector2<Float>;
pub type Vector3f = Vector3<Float>;
pub type Vector3fi = Vector3<Interval>;

pub type Bounds2f = Bounds2<Float>;
