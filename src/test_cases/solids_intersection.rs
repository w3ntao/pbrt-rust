use crate::fundamental::point::Point;
use crate::fundamental::vector::Vector;

use crate::ray_tracing::solids::triangle::Triangle;
use crate::ray_tracing::solids::sphere::Sphere;
use crate::ray_tracing::solids::axis_aligned_box::AxisAlignedBox;
use crate::ray_tracing::solids::quad::Quad;

use crate::ray_tracing::group::group_trait::GroupTrait;
use crate::ray_tracing::group::simple_group::SimpleGroup;

use crate::ray_tracing::cameras::perspective::PerspectiveCamera;
use crate::ray_tracing::integrators::ray_casting::RayCastingIntegrator;
use crate::ray_tracing::renderer::Renderer;
use crate::ray_tracing::world::World;

pub fn test() {
    let camera = PerspectiveCamera::new(
        Point::new(0.0, 0.0, 10.0),
        Vector::new(0.0, 0.0, -1.0),
        Vector::new(0.0, 1.0, 0.0),
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

    let sphere_0 = Sphere::new(Point::new(-2.0, 1.7, 0.0), 2.0);
    let sphere_1 = Sphere::new(Point::new(1.0, -1.0, 1.0), 2.2);
    let sphere_2 = Sphere::new(Point::new(3.0, 0.8, -2.0), 2.0);

    let quad = Quad::new(Point::new(1.0, -0.9, 4.5), Vector::new(-2.0, 0.0, 0.0), Vector::new(0.0, 0.1, -2.0));
    let aabox = AxisAlignedBox::new(Point::new(2.0, 1.5, -0.5), Point::new(3.0, 2.5, 2.5));

    let mut scene = SimpleGroup::new();
    scene.add(&triangle_0);
    scene.add(&triangle_1);

    scene.add(&sphere_0);
    scene.add(&sphere_1);
    scene.add(&sphere_2);

    scene.add(&quad);
    scene.add(&aabox);
    let scene = scene;

    let world = World::new(&scene);
    let integrator = RayCastingIntegrator::new(&world);
    let renderer = Renderer::new(&camera, &integrator);
    let image = renderer.render(640, 480);
    image.write("out.ppm");
}
