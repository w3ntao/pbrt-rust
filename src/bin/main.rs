use pbrt_minus::*;

fn render(file_path: &str) {
    let start = Instant::now();
    let mut builder = SceneBuilder::default();
    let mut scene_config = builder.parse_scene(file_path);

    let preprocessing_finished = Instant::now();

    let num_samples = 10;
    let cpu_num = num_cpus::get();

    scene_config.render(num_samples, cpu_num);
    println!(
        "total times: ({} + {}) second ({} cores)",
        (preprocessing_finished - start).as_secs(),
        preprocessing_finished.elapsed().as_secs(),
        cpu_num
    );
    println!();
}

fn main() {
    render("/home/wentao/Desktop/pbrt-v4-scenes-json/ganesha/ganesha.json");
    render("/home/wentao/Desktop/pbrt-v4-scenes-json/lte-orb/lte-orb-simple-ball.json");
    render("/home/wentao/Desktop/pbrt-v4-scenes-json/killeroos/killeroo-gold.json");
    render("/home/wentao/Desktop/pbrt-v4-scenes-json/killeroos/killeroo-simple.json");
    render("/home/wentao/Desktop/pbrt-v4-scenes-json/lte-orb/lte-orb-silver.json");
}
