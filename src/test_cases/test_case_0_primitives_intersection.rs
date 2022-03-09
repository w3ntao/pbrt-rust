use std::sync::Arc;

use crate::fundamental::point::Point;
use crate::fundamental::utility::get_file_name;
use crate::fundamental::vector3::Vector3;
use crate::ray_tracing::cameras::perspective::PerspectiveCamera;
use crate::ray_tracing::groups::simple_group::SimpleGroup;
use crate::ray_tracing::integrators::ray_casting::RayCastingIntegrator;
use crate::ray_tracing::materials::null::NullMaterial;
use crate::ray_tracing::primitives::axis_aligned_box::AxisAlignedBox;
use crate::ray_tracing::primitives::quad::Quad;
use crate::ray_tracing::primitives::sphere::Sphere;
use crate::ray_tracing::primitives::triangle::Triangle;
use crate::ray_tracing::renderer::Renderer;
use crate::ray_tracing::world::World;

pub fn test() {
    let file_name = get_file_name(file!());
    println!("TEST 0: {}", &file_name);
    let camera = PerspectiveCamera::new(
        Point::new(0.0, 0.0, 10.0),
        Vector3::new(0.0, 0.0, -1.0),
        Vector3::new(0.0, 1.0, 0.0),
        std::f32::consts::PI / 4.0,
        std::f32::consts::PI / 3.0);

    let triangle_0 = Triangle::new(
        Point::new(-2.0, 3.7, 0.0),
        Point::new(1.0, 2.0, 1.0),
        Point::new(3.0, 2.8, -2.0));
    let triangle_1 = Triangle::new(
        Point::new(3.0, 2.0, 3.0),
        Point::new(3.0, 2.0, -3.0),
        Point::new(-3.0, 2.0, -3.0));

    let sphere_0 = Sphere::new(Point::new(-2.0, 1.7, 0.0), 2.0, Arc::new(NullMaterial {}));
    let sphere_1 = Sphere::new(Point::new(1.0, -1.0, 1.0), 2.2, Arc::new(NullMaterial {}));
    let sphere_2 = Sphere::new(Point::new(3.0, 0.8, -2.0), 2.0, Arc::new(NullMaterial {}));

    let quad = Quad::new(Point::new(1.0, -0.9, 4.5), Vector3::new(-2.0, 0.0, 0.0), Vector3::new(0.0, 0.1, -2.0));
    let aabox = AxisAlignedBox::new(Point::new(2.0, 1.5, -0.5), Point::new(3.0, 2.5, 2.5));

    let mut scene = SimpleGroup::new();
    scene.add(Arc::new(triangle_0));
    scene.add(Arc::new(triangle_1));

    scene.add(Arc::new(sphere_0));
    scene.add(Arc::new(sphere_1));
    scene.add(Arc::new(sphere_2));

    scene.add(Arc::new(quad));
    scene.add(Arc::new(aabox));
    let scene = scene;

    let world = World::new(Arc::new(scene));
    let integrator = RayCastingIntegrator::new(Arc::new(world));
    let renderer = Renderer::new(Arc::new(camera), Arc::new(integrator), 1);
    let image = renderer.render(640, 480);

    image.write(&format!("{}.ppm", file_name));
    println!();
}
