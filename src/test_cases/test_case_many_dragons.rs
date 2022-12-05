use crate::core::pbrt::*;

#[allow(dead_code)]
pub fn test(width: usize, height: usize) {
    let file_name = get_file_name(file!());
    println!("TESTING: {}", &file_name);

    let glass = Arc::new(Glass::new(1.5));
    let dragon_model = Arc::new(load_dragon(glass.clone()));

    let num = 2000;
    let ratio: f32 = 20000.0;
    let delta = 0.2;

    let mut world = World::default();
    for idx in 0..num {
        let theta = (idx as f32) * delta;
        let mut dragon_instance = TransformedPrimitive::new(dragon_model.clone());
        dragon_instance.scale_by_scalar(ratio * (1.0 + 0.04 * idx as f32));
        dragon_instance.rotate(Vector3::new(0.0, 1.0, 0.0), theta);
        let radius = ratio * (1.0 + 0.13 * idx as f32);
        dragon_instance.translate(Vector3::new(
            radius * theta.sin(),
            0.0,
            radius * theta.cos(),
        ));

        world.add(Arc::new(dragon_instance));
    }
    world.build_index();

    let origin = Point::new(-7.0, 4.0, 0.0);
    let focus = Point::new(0.0, 0.0, 0.0);
    let look_at = focus - origin;

    let camera = Perspective::new(
        origin + (-ratio * look_at * 10.0),
        look_at,
        Vector3::new(0.0, 1.0, 0.0),
        PI / 6.0,
        (height as f32) / (width as f32),
    );

    let integrator = DebuggerRayCastingDotNormal::new(Arc::new(world));
    let renderer = Renderer::new(Arc::new(camera), Arc::new(integrator), 1);
    let image = renderer.render(width, height);
    image.write(&file_name);
    println!();
}
