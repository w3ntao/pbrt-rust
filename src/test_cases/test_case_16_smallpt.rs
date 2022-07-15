use std::sync::Arc;

use crate::fundamental::utility::*;
use crate::ray_tracing::cameras::perspective::Perspective;
use crate::ray_tracing::integrators::next_event_estimation::NextEventEstimation;
use crate::ray_tracing::integrators::ray_casting_dot_normal::RayCastingDotNormal;
use crate::ray_tracing::renderer::Renderer;
use crate::smallpt::smallpt;

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

    let camera_center = Point::new(50.0, 52.0, 295.6);
    //let look_at = Point::new(278.0, 278.0, 0.0);
    let direction = Vector3::new(0.0, -0.042612, -1.0);

    let camera = Perspective::new(
        camera_center,
        direction,
        Vector3::new(0.0, 1.0, 0.0),
        PI / 4.0,
        PI / 4.0);

    let integrator = RayCastingDotNormal::new(Arc::new(smallpt()));
    let renderer = Renderer::new(Arc::new(camera), Arc::new(integrator), samples);
    let image = renderer.render(WIDTH, HEIGHT);
    image.write(&ppm_name);
    println!();
}
