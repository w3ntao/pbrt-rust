use std::sync::Arc;

use crate::fundamental::point::Point;
use crate::fundamental::vector::Vector;
use crate::fundamental::obj_loader::obj_to_triangles;
use crate::fundamental::rgb_color::RGBColor;
use crate::fundamental::utility::get_file_name;

use crate::ray_tracing::group::Group;
use crate::ray_tracing::groups::bvh::BVH;
use crate::ray_tracing::instance::*;
use crate::ray_tracing::cameras::perspective::PerspectiveCamera;
use crate::ray_tracing::integrators::monte_carlo_path_trace::MonteCarloPathTrace;
use crate::ray_tracing::materials::null::NullMaterial;
use crate::ray_tracing::materials::lambertian::*;
use crate::ray_tracing::primitives::sphere::Sphere;
use crate::ray_tracing::renderer::Renderer;
use crate::ray_tracing::world::World;

pub fn test() {
    let file_name = get_file_name(file!());
    println!("TEST 3: {}", &file_name);
    let ppm_name = format!("test_3_{}.ppm", file_name);

    let material_ground = Lambertian { albedo: RGBColor::new(0.8, 0.8, 0.0) };
    let material_center = Lambertian { albedo: RGBColor::new(0.1, 0.2, 0.5) };

    let sphere_ground = Sphere::new(Point::new(0.0, -100.5, -1.0), 100.0, Arc::new(material_ground));
    let sphere_center = Sphere::new(Point::new(0.0, 0.0, -1.0), 0.5, Arc::new(material_center));

    let mut scene = BVH::default();
    scene.add(Arc::new(sphere_ground));
    scene.add(Arc::new(sphere_center));
    scene.build_index();

    let camera = PerspectiveCamera::new(
        Point::new(-2.0, 2.0, 1.0),
        Vector::new(2.0, -2.0, -2.0),
        Vector::new(0.0, 1.0, 0.0),
        std::f32::consts::PI / 8.0,
        std::f32::consts::PI / 6.0);

    let world = World::new(Arc::new(scene));
    let integrator = MonteCarloPathTrace::new(Arc::new(world));
    let renderer = Renderer::new(Arc::new(camera), Arc::new(integrator));
    let image = renderer.render(2000, 1500);
    image.write(&ppm_name);
    println!();
}
