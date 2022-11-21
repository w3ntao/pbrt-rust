use crate::core::pbrt::*;

fn random_color() -> Color {
    return Color::new(
        random_f32(0.0, 1.0),
        random_f32(0.0, 1.0),
        random_f32(0.0, 1.0),
    );
}

pub fn many_random_spheres() -> World {
    let mut scene = World::default();
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

            if choose_material < 0.4 {
                //diffuse
                let albedo = random_color() * random_color();
                let texture = Arc::new(SolidColor::new(albedo));
                let lambertian = Lambertian::new(texture.clone());
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

    return scene;
}

#[allow(dead_code)]
pub fn test(width: usize, height: usize, samples: u32) {
    let samples = ((samples as f32).sqrt() as u32).pow(2);
    let file_name = get_file_name(file!());
    println!("TESTING: {} for {} samples per pixel", &file_name, samples);

    let mut scene = many_random_spheres();
    let solid_color_ground = Arc::new(SolidColor::new(Color::new(0.5, 0.5, 0.5)));
    let material_ground = Arc::new(Lambertian::new(solid_color_ground.clone()));

    let ground_radius = 2000.0;
    let sphere_ground = Sphere::new(Point::new(0.0, -ground_radius, 0.0), ground_radius);
    let ground = Arc::new(GeometricPrimitive::new(
        Arc::new(sphere_ground),
        material_ground,
    ));

    scene.add(ground.clone());

    let texture_lambertian = Arc::new(SolidColor::new(Color::new(0.4, 0.2, 0.1)));
    let lambertian = Arc::new(Lambertian::new(texture_lambertian.clone()));
    let glass = Arc::new(Glass::new(1.5));
    let metal = Arc::new(Metal::new(Color::new(0.7, 0.6, 0.5), 0.0));

    let sphere_far = Sphere::new(Point::new(-4.0, 1.0, 0.0), 1.0);
    let sphere_far = Arc::new(GeometricPrimitive::new(
        Arc::new(sphere_far),
        lambertian.clone(),
    ));

    scene.add(sphere_far);

    let sphere_middle = Sphere::new(Point::new(0.0, 1.0, 0.0), 1.0);
    let sphere_middle = Arc::new(GeometricPrimitive::new(
        Arc::new(sphere_middle),
        glass.clone(),
    ));

    scene.add(sphere_middle);

    let center_sphere_close = Point::new(4.0, 1.0, 0.0);
    let sphere_near = Sphere::new(center_sphere_close, 1.0);
    let sphere_near = Arc::new(GeometricPrimitive::new(
        Arc::new(sphere_near),
        metal.clone(),
    ));

    scene.add(sphere_near);

    scene.build_index();

    let camera_center = Point::new(13.0, 2.0, 3.0);
    let look_at = Point::new(0.0, 0.0, 0.0);
    let direction = look_at - camera_center;

    let camera = DepthOfField::new(
        camera_center,
        direction,
        Vector3::new(0.0, 1.0, 0.0),
        PI / 6.0,
        (height as f32) / (width as f32),
        0.2,
        (camera_center - center_sphere_close).length(),
    );

    let integrator = PathTrace::new(Arc::new(scene), Color::new(0.7, 0.8, 1.0));
    let renderer = Renderer::new(Arc::new(camera), Arc::new(integrator), samples);
    let image = renderer.render(width, height);
    image.write(&format!("{}_{}", file_name, samples));
    println!();
}
