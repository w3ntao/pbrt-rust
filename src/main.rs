#![feature(const_fn_floating_point_arithmetic)]
extern crate ply_rs;

mod accelerator;
mod base;
mod cameras;
mod integrators;
mod math;
mod pbrt;
mod primitives;
mod samplers;
mod scene;
mod shapes;

use crate::pbrt::*;

fn render(file_path: &str) {
    let mut builder = SceneBuilder::default();
    let mut scene_config = builder.parse_scene(file_path);
    scene_config.render();
    println!();
}

fn main() {
    let path = "/home/wentao/Desktop/pbrt-v4-scenes-json/ganesha/geometry/ganesha.ply";

    read_ply(path);
    return;

    render("/home/wentao/Desktop/pbrt-v4-scenes-json/lte-orb/lte-orb-silver.json");
}
