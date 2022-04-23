use std::sync::Arc;

use crate::fundamental::utility::*;
use crate::ray_tracing::cameras::depth_of_field::DepthOfField;
use crate::ray_tracing::integrators::monte_carlo_path_trace::MonteCarloPathTrace;
use crate::ray_tracing::renderer::Renderer;
use crate::ray_tracing::world::World;
use crate::test_case_4_material_b::scene_three_spheres;

pub fn test(samples: u32) {
    let file_name = get_file_name(file!());
    println!("TESTING: {}", &file_name);
    let ppm_name = format!("{}.ppm", file_name);

    const WIDTH: usize = 1000;
    const HEIGHT: usize = 750;

    let camera_center = Point::new(-3.0, 3.0, 2.0);
    let focus_point = Point::new(0.0, 0.0, -1.0);

    let camera = DepthOfField::new(
        camera_center,
        Vector3::new(2.0, -2.0, -2.0),
        Vector3::new(0.0, 1.0, 0.0),
        PI / 8.0,
        PI / 6.0,
        0.4, (focus_point - camera_center).length());

    let world = World::new(Arc::new(scene_three_spheres()));
    let integrator = MonteCarloPathTrace::new(Arc::new(world), Color::new(0.7, 0.8, 1.0));
    let renderer = Renderer::new(Arc::new(camera), Arc::new(integrator), samples);
    let image = renderer.render(WIDTH, HEIGHT);
    image.write(&ppm_name);
    println!();
}
