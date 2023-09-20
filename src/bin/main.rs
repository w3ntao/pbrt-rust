extern crate clap;

use clap::Parser;
use pbrt_minus::*;
use std::fs;
use std::path::PathBuf;

#[derive(Parser)] // requires `derive` feature
#[command(author, version, about, long_about = None)]
struct Cli {
    #[arg(long)]
    spp: Option<usize>,

    scene_file: PathBuf,
}

fn render(file_path: &str, spp: usize) {
    let start = Instant::now();
    let mut builder = SceneBuilder::default();
    let mut scene_config = builder.parse_scene(file_path);
    let preprocessing_finished = Instant::now();

    let cpu_num = num_cpus::get();
    scene_config.render(spp, cpu_num);
    println!(
        "total times: ({} + {}) second ({} spp with {} cores)",
        (preprocessing_finished - start).as_secs(),
        preprocessing_finished.elapsed().as_secs(),
        spp,
        cpu_num
    );
}

fn main() {
    /*
    let (scale, coefficients) = compute_spectrum_table_data("sRGB");

    let rgb_to_spectrum_table = RGBtoSpectrumTable::new(scale, &coefficients);

    return;
    */

    let args = Cli::parse();
    if !args.scene_file.is_file() {
        panic!("`{}` is not a file", args.scene_file.display().to_string());
    }

    let absolute_path = fs::canonicalize(args.scene_file).unwrap();

    let spp = match args.spp {
        None => 16,
        Some(x) => x,
    };

    render(&absolute_path.display().to_string(), spp);
}
