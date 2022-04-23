use std::sync::Arc;

use crate::fundamental::obj_loader::obj_to_triangles;
use crate::fundamental::utility::*;
use crate::ray_tracing::cameras::perspective::Perspective;
use crate::ray_tracing::group::Group;
use crate::ray_tracing::groups::bvh::BVH;
use crate::ray_tracing::integrators::ray_casting_dot_normal::RayCastingDotNormal;
use crate::ray_tracing::renderer::Renderer;
use crate::ray_tracing::world::World;

pub fn test() {
    let file_name = get_file_name(file!());
    println!("TESTING: {}", &file_name);
    let triangles = obj_to_triangles("models/dragon.obj");
    let mut scene = BVH::default();
    for t in triangles {
        scene.add(t);
    }
    scene.build_index();

    let camera = Perspective::new(
        Point::new(-2.2, 0.0, 0.0),
        Vector3::new(1.0, 0.0, 0.0),
        Vector3::new(0.0, 1.0, 0.0),
        PI / 8.0,
        PI / 6.0);

    let world = World::new(Arc::new(scene));
    let integrator = RayCastingDotNormal::new(Arc::new(world));
    let renderer = Renderer::new(Arc::new(camera), Arc::new(integrator), 1);
    let image = renderer.render(2000, 1500);
    image.write(&format!("{}.ppm", file_name));
    println!();
}
