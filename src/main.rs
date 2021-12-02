mod ray;
mod intersection;
mod perspective_camera;
mod ray_casting_integrator;
mod group;
mod renderer;
mod solid;
mod primitive;
mod fundamental;

use crate::group::Group;
use fundamental::image::Image;
use crate::perspective_camera::PerspectiveCamera;
use crate::ray_casting_integrator::RayCastingIntegrator;
use crate::renderer::Renderer;
use solid::triangle::Triangle;
use crate::solid::sphere::Sphere;
use fundamental::point::Point;
use fundamental::vector::Vector;
use crate::solid::axis_aligned_box::AxisAlignedBox;
use crate::solid::quad::Quad;

fn main() {
    let camera = PerspectiveCamera::new(
        Point::new(0.0, 0.0, 10.0),
        Vector::new(0.0, 0.0, -1.0),
        Vector::new(0.0, 1.0, 0.0),
        std::f32::consts::PI / 4.0,
        std::f32::consts::PI / 3.0);

    let mut world = Group::new();
    world.add(Box::new(Triangle::new(
        Point::new(-2.0, 3.7, 0.0),
        Point::new(1.0, 2.0, 1.0),
        Point::new(3.0, 2.8, -2.0))));
    world.add(Box::new(Triangle::new(
        Point::new(3.0, 2.0, 3.0),
        Point::new(3.0, 2.0, -3.0),
        Point::new(-3.0, 2.0, -3.0))));

    world.add(Box::new(Sphere::new(Point::new(-2.0, 1.7, 0.0), 2.0)));
    world.add(Box::new(Sphere::new(Point::new(1.0, -1.0, 1.0), 2.2)));
    world.add(Box::new(Sphere::new(Point::new(3.0, 0.8, -2.0), 2.0)));

    world.add(Box::new(Quad::new(Point::new(1.0, -0.9, 4.5), Vector::new(-2.0, 0.0, 0.0), Vector::new(0.0, 0.1, -2.0))));

    world.add(Box::new(AxisAlignedBox::new(Point::new(2.0, 1.5, -0.5), Point::new(3.0, 2.5, 2.5))));

    let world = world;

    let integrator = RayCastingIntegrator::new(world);
    let renderer = Renderer::new(camera, integrator);
    let image = renderer.render(640, 480);
    image.write("out.ppm");
}
