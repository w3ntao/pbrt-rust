#![feature(const_fn_floating_point_arithmetic)]

mod base;
mod math;
mod pbrt;
mod scene;
mod shapes;

use crate::pbrt::*;

fn main() {
    let mut builder = SceneBuilder::new("killeroo-floor.json");
    let mut scene = builder.build_scene();
    scene.render();
}
