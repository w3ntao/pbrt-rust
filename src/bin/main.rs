use pbrt_minus::*;

fn render(file_path: &str) {
    let start = Instant::now();
    let mut builder = SceneBuilder::default();
    let mut scene_config = builder.parse_scene(file_path);

    let preprocessing_finished = Instant::now();

    let cores = scene_config.render();

    println!(
        "total times: ({} + {}) second ({} cores)",
        (preprocessing_finished - start).as_secs(),
        preprocessing_finished.elapsed().as_secs(),
        cores
    );
    println!();
}

fn main() {
    render("/home/wentao/Desktop/pbrt-v4-scenes-json/ganesha/ganesha.json");
    render("/home/wentao/Desktop/pbrt-v4-scenes-json/lte-orb/lte-orb-simple-ball.json");
    render("/home/wentao/Desktop/pbrt-v4-scenes-json/killeroos/killeroo-simple.json");

    return;

    render("/home/wentao/Desktop/pbrt-v4-scenes-json/lte-orb/lte-orb-silver.json");
    render("/home/wentao/Desktop/pbrt-v4-scenes-json/killeroos/killeroo-gold.json");
    render("/home/wentao/Desktop/pbrt-v4-scenes-json/killeroos/killeroo-simple.json");

    return;
}
