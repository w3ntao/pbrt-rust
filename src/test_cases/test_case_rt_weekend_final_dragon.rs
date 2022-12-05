use crate::core::pbrt::*;

fn random_bright_color() -> Color {
    loop {
        let color = Color::new(
            random_f32(0.0, 1.0),
            random_f32(0.0, 1.0),
            random_f32(0.0, 1.0),
        );

        if color.r > 0.6 || color.g > 0.6 || color.b > 0.6 {
            return color;
        }
    }
}

fn many_random_spheres_with_dragons() -> Scene {
    let mut scene = Scene::default();
    for a in -11..11 {
        let a = a as f32;
        for b in -11..11 {
            let b = b as f32;
            let choose_material = random_f32(0.0, 1.0);
            let center = Point::new(
                a + 0.9 * random_f32(0.0, 1.0),
                0.2,
                b + 0.9 * random_f32(0.0, 1.0),
            );

            let mut too_close = false;
            for point in [
                Point::new(-4.0, 0.2, 0.0),
                Point::new(0.0, 0.2, 0.0),
                Point::new(4.0, 0.2, 0.0),
            ] {
                if (center - point).length() <= 1.2 {
                    too_close = true;
                    break;
                }
            }

            if too_close {
                continue;
            }

            let sphere = Sphere::new(center, 0.2);
            let glass = Arc::new(Glass::new(1.5));

            let mut sphere = GeometricPrimitive::new(Arc::new(sphere), glass.clone());

            if choose_material < 0.5 {
                //diffuse
                let lambertian = Lambertian::new(random_bright_color());
                sphere.set_material(Arc::new(lambertian));
            } else if choose_material < 0.7 {
                // metal
                let albedo = random_f32(0.5, 1.0) * Color::new(1.0, 1.0, 1.0);
                let fuzz = random_f32(0.0, 0.5);
                let metal = Metal::new(albedo, fuzz);
                sphere.set_material(Arc::new(metal));
            } else {
                //glass
                sphere.set_material(glass);
            }

            scene.add(Arc::new(sphere));
        }
    }

    let solid_color_ground = Color::new(0.5, 0.5, 0.5);
    let material_ground = Arc::new(Lambertian::new(solid_color_ground));

    let length = 40.0;
    let quad = Quad::new(
        Point::new(-length / 2.0, 0.0, -length / 2.0),
        Vector3::new(length, 0.0, 0.0),
        Vector3::new(0.0, 0.0, length),
    );
    let ground = Arc::new(GeometricPrimitive::new(
        Arc::new(quad),
        material_ground.clone(),
    ));
    scene.add(ground.clone());

    let texture_lambertian = Color::new(0.4, 0.2, 0.1);
    let lambertian = Arc::new(Lambertian::new(texture_lambertian));
    let glass = Arc::new(Glass::new(1.5));
    let metal = Arc::new(Metal::new(Color::new(0.7, 0.6, 0.5), 0.0));

    let mut scaled_dragon = TransformedPrimitive::new(Arc::new(load_dragon(lambertian.clone())));
    scaled_dragon.rotate(Vector3::new(0.0, 1.0, 0.0), PI);
    scaled_dragon.scale_by_scalar(2.5);
    scaled_dragon.translate(Vector3::new(0.0, -scaled_dragon.get_bounds().p_min.y, 0.0));
    let dragon_instance = Arc::new(scaled_dragon);

    let mut dragon_instance_0 = TransformedPrimitive::new(dragon_instance.clone());
    dragon_instance_0.set_material(lambertian.clone());
    dragon_instance_0.translate(Vector3::new(-4.0, 0.0, 0.0));
    scene.add(Arc::new(dragon_instance_0));

    let mut dragon_instance_1 = TransformedPrimitive::new(dragon_instance.clone());
    dragon_instance_1.set_material(glass.clone());
    dragon_instance_1.translate(Vector3::new(0.0, 0.0, 0.0));
    scene.add(Arc::new(dragon_instance_1));

    let mut dragon_instance_2 = TransformedPrimitive::new(dragon_instance.clone());
    dragon_instance_2.set_material(metal.clone());
    dragon_instance_2.translate(Vector3::new(4.0, 0.0, 0.0));
    scene.add(Arc::new(dragon_instance_2));

    scene.build_index();
    return scene;
}

#[allow(dead_code)]
pub fn test(width: usize, height: usize, samples: u32) {
    let samples = ((samples as f32).sqrt() as u32).pow(2);
    let file_name = get_file_name(file!());
    println!("TESTING: {} for {} samples per pixel", &file_name, samples);

    let camera_center = Point::new(13.0, 2.0, 3.0);
    let look_at = Point::new(0.0, 0.0, 0.0);
    let direction = look_at - camera_center;

    let middle_dragon_center = Point::new(0.0, 1.0, 0.0);
    let camera = DepthOfField::new(
        camera_center,
        direction,
        Vector3::new(0.0, 1.0, 0.0),
        PI / 6.0,
        (height as f32) / (width as f32),
        0.15,
        (camera_center - middle_dragon_center).length(),
    );

    let integrator = PathTrace::new(Color::new(0.7, 0.8, 1.0));
    let renderer = Renderer::new(
        Arc::new(many_random_spheres_with_dragons()),
        Arc::new(camera),
        Arc::new(integrator),
        samples,
    );
    let image = renderer.render(width, height);
    image.write(&format!("{}_{}", file_name, samples));
    println!();
}
