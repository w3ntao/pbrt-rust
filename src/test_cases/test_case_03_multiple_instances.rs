use crate::cameras::perspective::Perspective;
use crate::core::pbrt::*;
use crate::integrators::ray_casting_dot_normal::RayCastingDotNormal;
use crate::utility::{get_file_name, load_dragon};
use std::sync::Arc;

#[allow(dead_code)]
pub fn test(width: usize, height: usize) {
    let file_name = get_file_name(file!());
    println!("TESTING: {}", &file_name);
    let dragon_model = Arc::new(load_dragon());

    let num = 5;
    let radius: f32 = 1.5;
    let delta = PI / (num as f32 - 1.0);

    let mut world = World::default();
    for idx in 0..num {
        let theta = (idx as f32) * delta;
        let mut dragon_instance = Instance::new(dragon_model.clone());
        dragon_instance.rotate(Vector3::new(0.0, 1.0, 0.0), theta);
        dragon_instance.translate(Vector3::new(
            radius * theta.sin(),
            0.0,
            radius * theta.cos(),
        ));
        world.add(Arc::new(dragon_instance));
    }
    world.build_index();

    let camera = Perspective::new(
        Point::new(-7.0, 5.0, 0.0),
        Vector3::new(1.0, -0.7, 0.0),
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
