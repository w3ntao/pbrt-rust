use std::sync::Arc;

use crate::fundamental::obj_loader::obj_to_triangles;
use crate::fundamental::point::Point;
use crate::fundamental::utility::get_file_name;
use crate::fundamental::vector3::Vector3;
use crate::ray_tracing::cameras::perspective::PerspectiveCamera;
use crate::ray_tracing::group::Group;
use crate::ray_tracing::groups::bvh::BVH;
use crate::ray_tracing::instance::*;
use crate::ray_tracing::integrators::ray_casting::RayCastingIntegrator;
use crate::ray_tracing::renderer::Renderer;
use crate::ray_tracing::world::World;

pub fn test() {
    let file_name = get_file_name(file!());
    println!("TEST 2: {}", &file_name);
    let triangles = obj_to_triangles("models/dragon.obj");
    let mut dragon_model = BVH::default();
    for t in triangles {
        dragon_model.add(t);
    }
    dragon_model.build_index();
    let dragon_model = Arc::new(dragon_model);

    let num = 5;
    let radius: f32 = 1.5;
    let delta = std::f32::consts::PI / (num as f32 - 1.0);

    let mut scene = BVH::default();
    for idx in 0..num {
        let theta = (idx as f32) * delta;
        let mut dragon_var = Instance::new(dragon_model.clone());
        dragon_var.rotate(Vector3::new(0.0, 1.0, 0.0), theta);
        dragon_var.translate(Vector3::new(radius * f32::sin(theta), 0.0, radius * f32::cos(theta)));
        scene.add(Arc::new(dragon_var));
    }
    scene.build_index();

    let camera = PerspectiveCamera::new(
        Point::new(-7.0, 5.0, 0.0),
        Vector3::new(1.0, -0.7, 0.0),
        Vector3::new(0.0, 1.0, 0.0),
        std::f32::consts::PI / 8.0,
        std::f32::consts::PI / 6.0);

    let world = World::new(Arc::new(scene));
    let integrator = RayCastingIntegrator::new(Arc::new(world));
    let renderer = Renderer::new(Arc::new(camera), Arc::new(integrator), 1);
    let image = renderer.render(2000, 1500);
    image.write(&format!("test_2_{}.ppm", file_name));
    println!();
}
