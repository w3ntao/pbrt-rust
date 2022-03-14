use std::sync::Arc;

use crate::fundamental::color::Color;
use crate::fundamental::obj_loader::obj_to_triangles;
use crate::fundamental::point::Point;
use crate::fundamental::utility::get_file_name;
use crate::fundamental::vector3::Vector3;
use crate::ray_tracing::cameras::depth_of_field::DepthOfField;
use crate::ray_tracing::cameras::perspective::PerspectiveCamera;
use crate::ray_tracing::group::Group;
use crate::ray_tracing::groups::bvh::BVH;
use crate::ray_tracing::instance::*;
use crate::ray_tracing::integrators::monte_carlo_path_trace::MonteCarloPathTrace;
use crate::ray_tracing::materials::glass::*;
use crate::ray_tracing::materials::lambertian::*;
use crate::ray_tracing::materials::metal::*;
use crate::ray_tracing::materials::mirror::*;
use crate::ray_tracing::primitive::Primitive;
use crate::ray_tracing::primitives::hollow_sphere::HollowSphere;
use crate::ray_tracing::primitives::sphere::Sphere;
use crate::ray_tracing::renderer::Renderer;
use crate::ray_tracing::world::World;

pub fn test(samples: i32) {
    let file_name = get_file_name(file!());
    println!("TESTING: {}", &file_name);
    let ppm_name = format!("{}.ppm", file_name);

    const WIDTH: usize = 1000;
    const HEIGHT: usize = 750;

    let material_ground = Arc::new(Lambertian { albedo: Color::new(0.8, 0.8, 0.0) });
    let material_center = Arc::new(Lambertian { albedo: Color::new(0.1, 0.2, 0.5) });
    let metal = Arc::new(Metal { albedo: Color::new(0.8, 0.6, 0.2), fuzz: 0.4 });
    let mirror = Arc::new(Mirror {});
    let glass = Arc::new(Glass::new());

    let mut sphere_ground = Sphere::new(Point::new(0.0, -100.5, -1.0), 100.0);
    sphere_ground.set_material(material_ground);

    let focus_point = Point::new(0.0, 0.0, -1.0);
    let radius = 0.5;
    let mut sphere_center = Sphere::new(focus_point.clone(), radius);
    sphere_center.set_material(material_center);

    let mut sphere_left = HollowSphere::new(Point::new(-1.0, 0.0, -1.0), radius, 0.05);
    sphere_left.set_material(glass);

    let mut sphere_right = Sphere::new(Point::new(1.0, 0.0, -1.0), radius);
    sphere_right.set_material(metal);

    let mut scene = BVH::default();
    scene.add(Arc::new(sphere_ground));
    scene.add(Arc::new(sphere_left));
    scene.add(Arc::new(sphere_center));
    scene.add(Arc::new(sphere_right));
    scene.build_index();

    let camera_center = Point::new(-3.0, 3.0, 2.0);

    let camera = DepthOfField::new(
        camera_center,
        Vector3::new(2.0, -2.0, -2.0),
        Vector3::new(0.0, 1.0, 0.0),
        std::f32::consts::PI / 8.0,
        std::f32::consts::PI / 6.0,
        0.4, (focus_point - camera_center).length());

    let world = World::new(Arc::new(scene));
    let integrator = MonteCarloPathTrace::new(Arc::new(world));
    let renderer = Renderer::new(Arc::new(camera), Arc::new(integrator), samples);
    let image = renderer.render(WIDTH, HEIGHT);
    image.write(&ppm_name);
    println!();
}
