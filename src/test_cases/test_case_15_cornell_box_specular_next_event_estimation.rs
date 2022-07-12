use std::sync::Arc;

use crate::fundamental::utility::*;
use crate::ray_tracing::cameras::perspective::Perspective;
use crate::ray_tracing::integrators::next_event_estimation_specular::NextEventEstimationSpecular;
use crate::ray_tracing::renderer::Renderer;
use crate::test_case_14_cornell_box_specular_monte_carlo::cornell_box_specular;

#[allow(dead_code)]
pub fn test(samples: u32) {
    let file_name = get_file_name(file!());

    println!("TESTING: {} for {} sampling (stratify)", &file_name, samples);
    let ppm_name = format!("{}_{}_stratify.ppm", file_name, samples);

    let samples_per_dimension = (samples as f32).sqrt() as u32;
    let samples = samples_per_dimension * samples_per_dimension;
    println!("actual samples: {}", samples);

    const WIDTH: usize = 600;
    const HEIGHT: usize = 600;

    let camera_center = Point::new(278.0, 278.0, -800.0);
    let look_at = Point::new(278.0, 278.0, 0.0);
    let direction = look_at - camera_center;

    let camera = Perspective::new(
        camera_center,
        direction,
        Vector3::new(0.0, 1.0, 0.0),
        PI / 4.0,
        PI / 4.0);

    let integrator = NextEventEstimationSpecular::new(Arc::new(cornell_box_specular()));
    let renderer = Renderer::new(Arc::new(camera), Arc::new(integrator), samples);
    let image = renderer.render(WIDTH, HEIGHT);
    image.write(&ppm_name);
    println!();
}
