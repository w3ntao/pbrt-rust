use crate::core::pbrt::*;

pub fn far_camera(width: usize, height: usize) -> Perspective {
    let offset = 1200.0;

    let camera = Perspective::new(
        Point::new(-2.2 - offset, 0.0, 0.0),
        Vector3::new(1.0, 0.0, 0.0),
        Vector3::new(0.0, 1.0, 0.0),
        PI / 6.0,
        (height as f32) / (width as f32),
    );
    return camera;
}

pub fn transformed_dragon() -> Scene {
    let mut scene = Scene::default();
    let glass = Arc::new(Glass::new(1.0));
    let dragon_model = Arc::new(load_dragon(glass.clone()));
    let mut dragon_instance = TransformedPrimitive::new(dragon_model.clone());

    dragon_instance.scale_by_scalar(400.0);

    scene.add(Arc::new(dragon_instance));
    scene.build_index();

    return scene;
}

#[allow(dead_code)]
pub fn test(width: usize, height: usize) {
    let file_name = get_file_name(file!());
    println!("TESTING: {}", &file_name);

    let integrator = DebuggerRayCastingDotNormal::default();
    let renderer = Renderer::new(
        Arc::new(transformed_dragon()),
        Arc::new(far_camera(width, height)),
        Arc::new(integrator),
        Arc::new(StratifiedSampler::default()),
        1,
    );
    let image = renderer.render(width, height);
    image.write(&file_name);
    println!();
}
