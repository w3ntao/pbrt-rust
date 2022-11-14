use crate::core::pbrt::*;

#[allow(dead_code)]
pub fn test(width: usize, height: usize) {
    let file_name = get_file_name(file!());
    let ppm_name = format!("{}.ppm", file_name);
    println!("TESTING: {}", &file_name);

    let glass = Arc::new(Glass::new(1.5));
    let mut scene = World::default();

    let ground_radius = 2000.0;
    let sphere_ground = Sphere::new(Point::new(0.0, -ground_radius, 0.0), ground_radius);
    let ground = Arc::new(GeometricPrimitive::new(
        Arc::new(sphere_ground),
        glass.clone(),
    ));
    scene.add(ground.clone());

    let mut scaled_dragon = TransformedPrimitive::new(Arc::new(load_dragon(glass.clone())));
    scaled_dragon.rotate(Vector3::new(0.0, 1.0, 0.0), PI);
    scaled_dragon.scale_by_scalar(2.5);
    scaled_dragon.translate(Vector3::new(0.0, -scaled_dragon.get_bounds().min.y, 0.0));
    let dragon_instance = Arc::new(scaled_dragon);

    for idx in 0..10 {
        let mut transformed_dragon = TransformedPrimitive::new(dragon_instance.clone());
        transformed_dragon.set_material(glass.clone());
        transformed_dragon.translate(Vector3::new(8.0 - 4.0 * (idx as f32), 0.0, 0.0));
        scene.add(Arc::new(transformed_dragon));
    }

    scene.build_index();

    let camera_center = Point::new(13.0, 2.0, 3.0);
    let look_at = Point::new(0.0, 0.0, 0.0);
    let direction = look_at - camera_center;

    let camera = Perspective::new(
        camera_center,
        direction,
        Vector3::new(0.0, 1.0, 0.0),
        PI / 6.0,
        (height as f32) / (width as f32),
    );

    let integrator = RayCastingDotNormal::new(Arc::new(scene));
    let renderer = Renderer::new(Arc::new(camera), Arc::new(integrator), 1);
    let image = renderer.render(width, height);
    image.write(&ppm_name);
    println!();
}
