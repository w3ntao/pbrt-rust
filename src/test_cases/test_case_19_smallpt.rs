use crate::core::renderer::Renderer;
use crate::fundamental::utility::*;
use crate::ray_tracing::cameras::perspective::Perspective;
use crate::ray_tracing::integrators::next_event_estimation::NextEventEstimation;
use crate::smallpt::smallpt;
use crate::utility::get_file_name;
use std::sync::Arc;

#[allow(dead_code)]
pub fn test(width: usize, height: usize, samples: u32) {
    let samples = ((samples as f32).sqrt() as u32).pow(2);
    let file_name = get_file_name(file!());
    let ppm_name = format!("{}_{}.ppm", file_name, samples);
    println!("TESTING: {} for {} samples per pixel", &file_name, samples);

    let camera_center = Point::new(50.0, 52.0, 275.6);
    let direction = Vector3::new(0.0, -0.042612, -1.0);

    let camera = Perspective::new(
        camera_center,
        direction,
        Vector3::new(0.0, 1.0, 0.0),
        PI / 4.3,
        (height as f32) / (width as f32),
    );

    let integrator = NextEventEstimation::new(Arc::new(smallpt()));
    let renderer = Renderer::new(Arc::new(camera), Arc::new(integrator), samples);
    let image = renderer.render(width, height);
    image.write(&ppm_name);
    println!();
}
