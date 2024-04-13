extern crate clap;

use clap::Parser;
use pbrt_rust::*;
use std::fs;
use std::path::PathBuf;

#[derive(Parser)] // requires `derive` feature
#[command(author, version, about, long_about = None)]
struct Cli {
    #[arg(long)]
    spp: Option<usize>,
    scene_file: PathBuf,
}

fn render(file_path: &str, samples_per_pixel: usize) {
    let start = Instant::now();

    let mut builder = SceneBuilder::default();
    let mut scene_config = builder.parse_scene(file_path, samples_per_pixel);

    println!(
        "preprocessing (spectra computing + BVH building): {:.2} seconds",
        start.elapsed().as_secs_f32(),
    );

    let cpu_num = num_cpus::get();

    scene_config.render(samples_per_pixel, cpu_num);
}

fn main() {
    let args = Cli::parse();
    if !args.scene_file.is_file() {
        panic!("`{}` is not a file", args.scene_file.display().to_string());
    }

    let absolute_path = fs::canonicalize(args.scene_file).unwrap();

    let spp = args.spp.unwrap_or_else(|| 32);

    render(&absolute_path.display().to_string(), spp);
}
