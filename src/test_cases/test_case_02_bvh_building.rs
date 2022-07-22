use crate::fundamental::utility::*;
use crate::ray_tracing::cameras::perspective::Perspective;
use crate::ray_tracing::instance::Instance;
use crate::ray_tracing::integrators::ray_casting_dot_normal::RayCastingDotNormal;
use crate::ray_tracing::renderer::Renderer;
use crate::ray_tracing::world::World;
use crate::utility::{get_file_name, load_dragon};
use std::sync::Arc;

#[allow(dead_code)]
pub fn test(width: usize, height: usize) {
    let file_name = get_file_name(file!());
    println!("TESTING: {}", &file_name);

    let dragon_instance = Instance::new(Arc::new(load_dragon()));

    let mut world = World::default();
    world.add(Arc::new(dragon_instance));
    world.build_index();

    let camera = Perspective::new(
        Point::new(-2.2, 0.0, 0.0),
        Vector3::new(1.0, 0.0, 0.0),
        Vector3::new(0.0, 1.0, 0.0),
        PI / 6.0,
        (height as f32) / (width as f32),
    );

    let integrator = RayCastingDotNormal::new(Arc::new(world));
    let renderer = Renderer::new(Arc::new(camera), Arc::new(integrator), 1);
    let image = renderer.render(width, height);
    image.write(&format!("{}.ppm", file_name));
    println!();
}
