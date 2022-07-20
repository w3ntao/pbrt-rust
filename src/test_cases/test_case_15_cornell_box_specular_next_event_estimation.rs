use crate::cornell_box::{cornell_box_camera, cornell_box_specular};
use crate::fundamental::utility::*;
use crate::ray_tracing::integrators::next_event_estimation::NextEventEstimation;
use crate::ray_tracing::renderer::Renderer;
use std::sync::Arc;

#[allow(dead_code)]
pub fn test(width: usize, height: usize, samples: u32) {
    let file_name = get_file_name(file!());

    println!(
        "TESTING: {} for {} sampling (stratify)",
        &file_name, samples
    );
    let ppm_name = format!("{}_{}_stratify.ppm", file_name, samples);

    let samples_per_dimension = (samples as f32).sqrt() as u32;
    let samples = samples_per_dimension * samples_per_dimension;
    println!("actual samples: {}", samples);

    let integrator = NextEventEstimation::new(Arc::new(cornell_box_specular()));
    let renderer = Renderer::new(
        Arc::new(cornell_box_camera(width, height)),
        Arc::new(integrator),
        samples,
    );
    let image = renderer.render(width, height);
    image.write(&ppm_name);
    println!();
}
