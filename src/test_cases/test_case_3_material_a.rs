use std::sync::Arc;

use crate::fundamental::color::Color;
use crate::fundamental::point::Point;
use crate::fundamental::utility::get_file_name;
use crate::fundamental::vector3::Vector3;
use crate::ray_tracing::cameras::perspective::Perspective;
use crate::ray_tracing::group::Group;
use crate::ray_tracing::groups::bvh::BVH;
use crate::ray_tracing::integrators::monte_carlo_path_trace::MonteCarloPathTrace;
use crate::ray_tracing::materials::glass::*;
use crate::ray_tracing::materials::lambertian::*;
use crate::ray_tracing::materials::metal::*;
use crate::ray_tracing::materials::mirror::*;
use crate::ray_tracing::primitive::Primitive;
use crate::ray_tracing::primitives::sphere::Sphere;
use crate::ray_tracing::renderer::Renderer;
use crate::ray_tracing::textures::solid_color::SolidColor;
use crate::ray_tracing::world::World;

pub fn test(samples: u32) {
    let file_name = get_file_name(file!());
    println!("TESTING: {}", &file_name);
    let ppm_name = format!("{}.ppm", file_name);

    const WIDTH: usize = 1000;
    const HEIGHT: usize = 750;

    let material_ground = Arc::new(Lambertian::new(Arc::new(SolidColor::new(Color::new(0.8, 0.8, 0.0)))));
    let metal = Arc::new(Metal::new(Color::new(0.8, 0.6, 0.2), 0.4));
    let mirror = Arc::new(Mirror::new());
    let glass = Arc::new(Glass::new(1.5));

    let mut sphere_ground = Sphere::new(Point::new(0.0, -100.5, -1.0), 100.0);
    sphere_ground.set_material(material_ground);

    let mut sphere_center = Sphere::new(Point::new(0.0, 0.0, -1.0), 0.5);
    sphere_center.set_material(glass);

    let mut sphere_left = Sphere::new(Point::new(-1.0, 0.0, -1.0), 0.5);
    sphere_left.set_material(mirror);

    let mut sphere_right = Sphere::new(Point::new(1.0, 0.0, -1.0), 0.5);
    sphere_right.set_material(metal);

    let mut scene = BVH::default();
    scene.add(Arc::new(sphere_ground));
    scene.add(Arc::new(sphere_left));
    scene.add(Arc::new(sphere_center));
    scene.add(Arc::new(sphere_right));
    scene.build_index();

    let camera = Perspective::new(
        Point::new(0.0, 0.0, 5.0),
        Vector3::new(0.0, 0.0, -1.0),
        Vector3::new(0.0, 1.0, 0.0),
        std::f32::consts::PI / 8.0,
        std::f32::consts::PI / 6.0);

    let world = World::new(Arc::new(scene));
    let integrator = MonteCarloPathTrace::new(Arc::new(world));
    let renderer = Renderer::new(Arc::new(camera), Arc::new(integrator), samples);
    let image = renderer.render(WIDTH, HEIGHT);
    image.write(&ppm_name);
    println!();
}
