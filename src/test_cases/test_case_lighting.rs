use crate::core::pbrt::*;

#[allow(dead_code)]
pub fn test(width: usize, height: usize, samples: u32) {
    let samples = ((samples as f32).sqrt() as u32).pow(2);
    let file_name = get_file_name(file!());
    let ppm_name = format!("{}_{}.ppm", file_name, samples);
    println!("TESTING: {} for {} samples per pixel", &file_name, samples);

    let mut scene = World::default();

    let perlin_texture = Arc::new(NoiseTexture::new(4.0));
    let lambertian_perlin = Arc::new(Lambertian::new(perlin_texture.clone()));

    let mut big_sphere = Sphere::new(Point::new(0.0, -1000.0, 0.0), 1000.0);
    big_sphere.set_material(lambertian_perlin.clone());
    let big_sphere = Arc::new(GeometricPrimitive::new(Arc::new(big_sphere)));
    scene.add(big_sphere);

    let sphere_center = Point::new(0.0, 2.0, 0.0);
    let mut small_sphere = Sphere::new(sphere_center, 2.0);
    small_sphere.set_material(lambertian_perlin.clone());
    let small_sphere = Arc::new(GeometricPrimitive::new(Arc::new(small_sphere)));
    scene.add(small_sphere);

    let diffuse_light = DiffuseLight::new(Arc::new(SolidColor::new(Color::new(4.0, 4.0, 4.0))));
    let mut quad_light = Quad::new(
        Point::new(3.0, 1.0, -2.0),
        Vector3::new(2.0, 0.0, 0.0),
        Vector3::new(0.0, 2.0, 0.0),
    );
    quad_light.set_material(Arc::new(diffuse_light));
    let quad_light = Arc::new(GeometricPrimitive::new(Arc::new(quad_light)));
    scene.add(quad_light);

    scene.build_index();

    let camera_center = Point::new(26.0, 3.0, 6.0);
    let look_at = Point::new(0.0, 2.0, 0.0);
    let direction = look_at - camera_center;

    let camera = DepthOfField::new(
        camera_center,
        direction,
        Vector3::new(0.0, 1.0, 0.0),
        PI / 6.0,
        (height as f32) / (width as f32),
        0.000002,
        (look_at - camera_center).length(),
    );

    let integrator = PathTrace::new(Arc::new(scene), Color::new(0.0, 0.0, 0.0));
    let renderer = Renderer::new(Arc::new(camera), Arc::new(integrator), samples);
    let image = renderer.render(width, height);
    image.write(&ppm_name);
    println!();
}
