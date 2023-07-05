#![feature(const_fn_floating_point_arithmetic)]

mod base;
mod cameras;
mod integrators;
mod math;
mod pbrt;
mod samplers;
mod scene;
mod shapes;

use crate::pbrt::*;

fn main() {
    let mut builder = SceneBuilder::new("killeroo-floor.json");
    /*
    let mut builder =
        SceneBuilder::new("/home/wentao/Desktop/pbrt-v4-scenes-json/killeroos/killeroo-gold.json");
    */
    let mut scene_config = builder.build_scene();
    scene_config.render();
}
