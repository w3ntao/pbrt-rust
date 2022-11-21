use crate::core::pbrt::*;

#[allow(dead_code)]
pub fn test(width: usize, height: usize) {
    let file_name = get_file_name(file!());
    println!("TESTING: {}", &file_name);

    let mut world = World::default();
    let glass = Arc::new(Glass::new(1.5));
    let dragon_model = Arc::new(load_dragon(glass.clone()));
    let dragon_instance = TransformedPrimitive::new(dragon_model.clone());
    world.add(Arc::new(dragon_instance));
    world.build_index();

    let camera = Perspective::new(
        Point::new(-2.9, 0.0, 0.0),
        Vector3::new(1.0, 0.0, 0.0),
        Vector3::new(0.0, 1.0, 0.0),
        PI / 6.0,
        (height as f32) / (width as f32),
    );

    let integrator = RayCastingDotNormal::new(Arc::new(world));
    let renderer = Renderer::new(Arc::new(camera), Arc::new(integrator), 1);
    let image = renderer.render(width, height);
    image.write(&file_name);
    println!();
}
