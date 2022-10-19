use crate::core::pbrt::*;
use crate::cornell_box::{cornell_box, cornell_box_camera};
use crate::integrators::monte_carlo_path_trace::MonteCarloPathTrace;
use crate::utility::get_file_name;
use std::sync::Arc;

#[allow(dead_code)]
pub fn test(width: usize, height: usize, samples: u32) {
    let samples = ((samples as f32).sqrt() as u32).pow(2);
    let file_name = get_file_name(file!());
    let ppm_name = format!("{}_{}.ppm", file_name, samples);
    println!("TESTING: {} for {} samples per pixel", &file_name, samples);

    let integrator = MonteCarloPathTrace::new(Arc::new(cornell_box()), Color::black());
    let renderer = Renderer::new(
        Arc::new(cornell_box_camera(width, height)),
        Arc::new(integrator),
        samples,
    );
    let image = renderer.render(width, height);
    image.write(&ppm_name);
    println!();
}
