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

fn render(file_path: &str) {
    let mut builder = SceneBuilder::default();
    let mut scene_config = builder.parse_scene(file_path);
    scene_config.render();
}

fn main() {
    //render("killeroo-floor.json");
    render("/home/wentao/Desktop/pbrt-v4-scenes-json/killeroos/killeroo-gold.json");
}
