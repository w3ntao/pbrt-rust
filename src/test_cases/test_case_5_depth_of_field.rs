use std::sync::Arc;

use crate::fundamental::color::Color;
use crate::fundamental::point::Point;
use crate::fundamental::utility::get_file_name;
use crate::fundamental::vector3::Vector3;
use crate::ray_tracing::cameras::depth_of_field::DepthOfField;
use crate::ray_tracing::group::Group;
use crate::ray_tracing::groups::bvh::BVH;
use crate::ray_tracing::integrators::monte_carlo_path_trace::MonteCarloPathTrace;
use crate::ray_tracing::materials::glass::*;
use crate::ray_tracing::materials::lambertian::*;
use crate::ray_tracing::materials::metal::*;
use crate::ray_tracing::primitive::Primitive;
use crate::ray_tracing::primitives::hollow_sphere::HollowSphere;
use crate::ray_tracing::primitives::sphere::Sphere;
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
        std::f32::consts::PI / 8.0,
        std::f32::consts::PI / 6.0,
        0.4, (focus_point - camera_center).length());

    let world = World::new(Arc::new(scene_three_spheres()));
    let integrator = MonteCarloPathTrace::new(Arc::new(world));
    let renderer = Renderer::new(Arc::new(camera), Arc::new(integrator), samples);
    let image = renderer.render(WIDTH, HEIGHT);
    image.write(&ppm_name);
    println!();
}
