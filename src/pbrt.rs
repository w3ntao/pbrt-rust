pub extern crate ply_rs;

pub type Float = f32;
//pub type Float = f64;

pub enum RenderingCoordinateSystem {
    Camera,
    CameraWorld,
    World,
}

pub use fma::fma;
pub use ply_rs::ply;
pub use rand::{rngs::StdRng, thread_rng, Rng, SeedableRng};
pub use serde_json::Value;
pub use std::{
    any::type_name,
    collections::{HashMap, HashSet},
    convert::TryInto,
    fmt::{Debug, Display, Formatter},
    fs::File,
    hash::{Hash, Hasher},
    io::{BufReader, Read},
    iter::Sum,
    ops::{Add, AddAssign, Div, Index, IndexMut, Mul, MulAssign, Neg, Sub},
    path::Path,
    process::{exit, Command},
    str::FromStr,
    sync::{
        atomic::{AtomicUsize, Ordering},
        {Arc, Mutex},
    },
    thread,
    thread::JoinHandle,
    time::Instant,
};

pub use crate::{
    accelerator::bvh::*,
    base::{
        camera::*, film::*, filter::*, integrator::*, primitive::*, ray::*, rgb_color::*,
        sampler::*, shape::*, shape_intersection::*,
    },
    cameras::perspective::*,
    filters::box_filter::*,
    integrators::surface_normal_visualizer::*,
    math::{
        arithmetic::*, bounds::*, compensated_float::*, float::*, interval::*, interval::*,
        normal::*, point2::*, point3::*, square_matrix::*, transform::*, vector2::*, vector3::*,
    },
    primitives::simple_primitive::*,
    samplers::independent::*,
    scene::{parameter_dict::*, scene_builder::*, scene_config::*, util::*},
    shapes::{
        loop_subdivision::*, sphere::*, tri_quad_mesh::*, triangle::*, triangle_mesh::*, util::*,
    },
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

pub fn type_of<T>(_: T) -> &'static str {
    type_name::<T>()
}
