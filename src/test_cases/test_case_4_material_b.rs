use std::sync::Arc;

use crate::fundamental::color::Color;
use crate::fundamental::obj_loader::obj_to_triangles;
use crate::fundamental::point::Point;
use crate::fundamental::utility::get_file_name;
use crate::fundamental::vector3::Vector3;
use crate::ray_tracing::cameras::perspective::PerspectiveCamera;
use crate::ray_tracing::group::Group;
use crate::ray_tracing::groups::bvh::BVH;
use crate::ray_tracing::instance::*;
use crate::ray_tracing::integrators::monte_carlo_path_trace::MonteCarloPathTrace;
use crate::ray_tracing::materials::glass::*;
use crate::ray_tracing::materials::lambertian::*;
use crate::ray_tracing::materials::metal::*;
use crate::ray_tracing::materials::mirror::*;
use crate::ray_tracing::primitives::hollow_sphere::HollowSphere;
use crate::ray_tracing::primitives::sphere::Sphere;
use crate::ray_tracing::renderer::Renderer;
use crate::ray_tracing::world::World;

pub fn test() {
    let file_name = get_file_name(file!());
    println!("TEST 4: {}", &file_name);
    let ppm_name = format!("{}.ppm", file_name);

    const WIDTH: usize = 1000;
    const HEIGHT: usize = 750;
    const SAMPLES: i32 = 10;

    let material_ground = Arc::new(Lambertian { albedo: Color::new(0.8, 0.8, 0.0) });
    let material_center = Arc::new(Lambertian { albedo: Color::new(0.1, 0.2, 0.5) });
    let metal = Arc::new(Metal { albedo: Color::new(0.8, 0.6, 0.2), fuzz: 0.4 });
    let mirror = Arc::new(Mirror {});
    let glass = Arc::new(Glass::new());

    /*
    let sphere_ground = Sphere::new(Point::new(0.0, -100.5, -1.0), 100.0, material_ground.clone());
    let sphere_center = Sphere::new(Point::new(0.0, 0.0, -1.0), 0.5, material_center.clone());
    let sphere_left = HollowSphere::new(Point::new(-1.0, 0.0, -1.0), 0.5, 0.05, glass.clone());
    let sphere_right = Sphere::new(Point::new(1.0, 0.0, -1.0), 0.5, metal.clone());
     */

    let sphere_ground = Sphere::new(Point::new(0.0, -100.5, -1.0), 100.0);
    let sphere_center = Sphere::new(Point::new(0.0, 0.0, -1.0), 0.5);
    let sphere_left = HollowSphere::new(Point::new(-1.0, 0.0, -1.0), 0.5, 0.05);
    let sphere_right = Sphere::new(Point::new(1.0, 0.0, -1.0), 0.5);


    let mut scene = BVH::default();
    scene.add(Arc::new(sphere_ground));
    scene.add(Arc::new(sphere_left));
    scene.add(Arc::new(sphere_center));
    scene.add(Arc::new(sphere_right));
    scene.build_index();

    let camera = PerspectiveCamera::new(
        Point::new(-2.0, 2.0, 1.0),
        Vector3::new(2.0, -2.0, -2.0),
        Vector3::new(0.0, 1.0, 0.0),
        std::f32::consts::PI / 8.0,
        std::f32::consts::PI / 6.0);

    let world = World::new(Arc::new(scene));
    let integrator = MonteCarloPathTrace::new(Arc::new(world));
    let renderer = Renderer::new(Arc::new(camera), Arc::new(integrator), SAMPLES);
    let image = renderer.render(WIDTH, HEIGHT);
    image.write(&ppm_name);
    println!();
}
