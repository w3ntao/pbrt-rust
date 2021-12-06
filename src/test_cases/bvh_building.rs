use std::rc::Rc;
use crate::fundamental::point::Point;
use crate::fundamental::vector::Vector;
use crate::fundamental::obj_loader::obj_to_triangles;

use crate::ray_tracing::primitives::triangle::Triangle;
use crate::ray_tracing::primitives::sphere::Sphere;
use crate::ray_tracing::primitives::axis_aligned_box::AxisAlignedBox;
use crate::ray_tracing::primitives::quad::Quad;

use crate::ray_tracing::group::Group;
use crate::ray_tracing::groups::simple_group::SimpleGroup;
use crate::ray_tracing::groups::bvh::BVH;

use crate::ray_tracing::cameras::perspective::PerspectiveCamera;
use crate::ray_tracing::integrators::ray_casting::RayCastingIntegrator;
use crate::ray_tracing::renderer::Renderer;
use crate::ray_tracing::world::World;

pub fn test() {
    let triangles = obj_to_triangles("models/dragon.obj");
    let mut scene = BVH::default();
    for t in triangles {
        scene.add(t);
    }
    scene.build_index();

    let camera = PerspectiveCamera::new(
        Point::new(-2.2, 0.0, 0.0),
        Vector::new(1.0, 0.0, 0.0),
        Vector::new(0.0, 1.0, 0.0),
        std::f32::consts::PI / 8.0,
        std::f32::consts::PI / 6.0);

    let world = World::new(&scene);
    let integrator = RayCastingIntegrator::new(&world);
    let renderer = Renderer::new(&camera, &integrator);
    let image = renderer.render(2000, 1500);
    image.write("dragon.ppm");
}
