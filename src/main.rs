use crate::group::Group;
use crate::image::Image;
use crate::perspective_camera::PerspectiveCamera;
use crate::ray_casting_integrator::RayCastingIntegrator;
use crate::renderer::Renderer;
use crate::triangle::Triangle;
use crate::vector::Vector;

mod vector;
mod triangle;
mod ray;
mod intersection;
mod perspective_camera;
mod ray_casting_integrator;
mod group;
mod renderer;
mod image;
mod primitive;


fn main() {
    const IMAGE_WIDTH: usize = 640;
    const IMAGE_HEIGHT: usize = 480;

    let camera = PerspectiveCamera::new(
        Vector::new(0.0, 0.0, 10.0),
        Vector::new(0.0, 0.0, -1.0),
        Vector::new(0.0, 1.0, 0.0),
        std::f32::consts::PI / 4.0,
        std::f32::consts::PI / 3.0);

    let mut world = Group::new();
    world.add(Box::new(Triangle::new(
        Vector::new(-2.0, 3.7, 0.0),
        Vector::new(1.0, 2.0, 1.0),
        Vector::new(3.0, 2.8, -2.0))));
    world.add(Box::new(Triangle::new(
        Vector::new(3.0, 2.0, 3.0),
        Vector::new(3.0, 2.0, -3.0),
        Vector::new(-3.0, 2.0, -3.0))));

    let integrator = RayCastingIntegrator::new(world);
    let renderer = Renderer::new(camera, integrator);
    let image = renderer.render(IMAGE_WIDTH, IMAGE_HEIGHT);
    image.write("out.ppm");
}
