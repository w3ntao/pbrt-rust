use crate::core::renderer::Renderer;
use crate::fundamental::utility::*;
use crate::ray_tracing::cameras::depth_of_field::DepthOfField;
use crate::ray_tracing::integrators::monte_carlo_path_trace::MonteCarloPathTrace;
use crate::test_case_05_material_b::scene_three_spheres;
use crate::utility::get_file_name;
use std::sync::Arc;

#[allow(dead_code)]
pub fn test(width: usize, height: usize, samples: u32) {
    let samples = ((samples as f32).sqrt() as u32).pow(2);
    let file_name = get_file_name(file!());
    let ppm_name = format!("{}_{}.ppm", file_name, samples);
    println!("TESTING: {} for {} samples per pixel", &file_name, samples);

    let camera_center = Point::new(-3.0, 3.0, 2.0);
    let focus_point = Point::new(0.0, 0.0, -1.0);

    let camera = DepthOfField::new(
        camera_center,
        Vector3::new(2.0, -2.0, -2.0),
        Vector3::new(0.0, 1.0, 0.0),
        PI / 6.0,
        (height as f32) / (width as f32),
        0.4,
        (focus_point - camera_center).length(),
    );

    let integrator =
        MonteCarloPathTrace::new(Arc::new(scene_three_spheres()), Color::new(0.7, 0.8, 1.0));
    let renderer = Renderer::new(Arc::new(camera), Arc::new(integrator), samples);
    let image = renderer.render(width, height);
    image.write(&ppm_name);
    println!();
}
