#![feature(const_fn_floating_point_arithmetic)]

mod base;
mod cameras;
mod integrators;
mod math;
mod pbrt;
mod scene;
mod shapes;

use crate::pbrt::*;

fn main() {
    let mut builder = SceneBuilder::new("killeroo-floor.json");
    let mut scene_config = builder.build_scene();
    scene_config.render();
}
