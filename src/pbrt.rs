pub type Float = f32;
// pub type Float = f64;

pub use serde_json::Value;
pub use std::fs::File;
pub use std::io::Read;

pub use crate::point::*;
pub use crate::scene_builder::SceneBuilder;
pub use crate::square_matrix::SquareMatrix;

pub type Point2f = Point2<Float>;
pub type Point3f = Point3<Float>;
pub type Vector3f = Vector3<Float>;
