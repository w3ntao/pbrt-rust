use crate::core::pbrt::*;

pub fn bug_camera(width: usize, height: usize) -> Perspective {
    let offset = 800.0;

    let camera = Perspective::new(
        Point::new(-2.2 - offset, 0.0, 0.0),
        Vector3::new(1.0, 0.0, 0.0),
        Vector3::new(0.0, 1.0, 0.0),
        PI / 6.0,
        (height as f32) / (width as f32),
    );
    return camera;
}

pub fn bug_dragon() -> World {
    let mut world = World::default();
    let glass = Arc::new(Glass::new(1.0));
    let dragon_model = Arc::new(load_dragon(glass.clone()));
    let mut dragon_instance = TransformedPrimitive::new(dragon_model.clone());

    dragon_instance.scale_by_scalar(400.0);

    world.add(Arc::new(dragon_instance));
    world.build_index();

    return world;
}

#[allow(dead_code)]
pub fn test(width: usize, height: usize) {
    let file_name = get_file_name(file!());
    let ppm_name = format!("{}.ppm", file_name);
    println!("TESTING: {}", &file_name);

    let integrator = RayCastingDotNormal::new(Arc::new(bug_dragon()));
    let renderer = Renderer::new(Arc::new(bug_camera(width, height)), Arc::new(integrator), 1);
    let image = renderer.render(width, height);
    image.write(&ppm_name);
    println!();
}
