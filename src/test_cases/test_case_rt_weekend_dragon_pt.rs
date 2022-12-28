use crate::core::pbrt::*;

const RANDOM_SEED: u64 = 11;

fn random_color(random_generator: &mut StdRng) -> Color {
    let uniform_distribution = Uniform::new(0.0, 1.0);

    loop {
        let color = Color::new(
            uniform_distribution.sample(random_generator),
            uniform_distribution.sample(random_generator),
            uniform_distribution.sample(random_generator),
        );

        if color.r > 0.6 || color.g > 0.6 || color.b > 0.6 {
            return color;
        }
    }
}

pub fn many_random_spheres_with_dragons() -> Scene {
    let mut random_generator = StdRng::seed_from_u64(RANDOM_SEED);
    let uniform_distribution = Uniform::new(0.0, 1.0);

    let mut scene = Scene::default();

    let dragon_center_list = [
        Point::new(-4.0, 0.2, 0.0),
        Point::new(0.0, 0.2, 0.0),
        Point::new(4.0, 0.2, 0.0),
    ];
    let mut sphere_center_list: Vec<Point> = vec![];

    let radius = 0.2;

    for a in -11..11 {
        let a = a as f32;
        for b in -11..11 {
            let b = b as f32;
            let choose_material = uniform_distribution.sample(&mut random_generator);
            let center = Point::new(
                a + 0.9 * uniform_distribution.sample(&mut random_generator),
                radius,
                b + 0.9 * uniform_distribution.sample(&mut random_generator),
            );

            let mut too_close = false;
            for point in &dragon_center_list {
                too_close |= (center - *point).length() <= 1.5;
            }
            for point in &sphere_center_list {
                too_close |= (center - *point).length() <= radius * 2.3;
            }
            if too_close {
                continue;
            }
            sphere_center_list.push(center);

            let sphere = Sphere::new(center, radius);
            let glass = Arc::new(Glass::new(1.5));

            let mut sphere = GeometricPrimitive::new(Arc::new(sphere), glass.clone());

            if choose_material < 0.5 {
                //diffuse
                let lambertian = Lambertian::new(random_color(&mut random_generator));
                sphere.set_material(Arc::new(lambertian));
            } else if choose_material < 0.7 {
                // metal
                let albedo = (uniform_distribution.sample(&mut random_generator) * 0.5 + 0.5)
                    * Color::new(1.0, 1.0, 1.0);
                let fuzz = uniform_distribution.sample(&mut random_generator) * 0.5;

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

pub fn rt_weekend_camera(width: usize, height: usize) -> Arc<dyn Camera> {
    let camera_center = Point::new(13.0, 2.0, 3.0);
    let look_at = Point::new(0.0, 0.0, 0.0);
    let direction = look_at - camera_center;

    let middle_dragon_center = Point::new(0.0, 1.0, 0.0);
    let camera = Perspective::with_lens(
        camera_center,
        direction,
        Vector3::new(0.0, 1.0, 0.0),
        PI / 6.0,
        (height as f32) / (width as f32),
        0.15,
        (camera_center - middle_dragon_center).length(),
    );

    return Arc::new(camera);
}

#[allow(dead_code)]
pub fn test(width: usize, height: usize, samples: u32) {
    let samples = ((samples as f32).sqrt() as u32).pow(2);
    let file_name = get_file_name(file!());
    println!("TESTING: {} for {} samples per pixel", &file_name, samples);

    let integrator = PathTrace::new(Color::new(0.7, 0.8, 1.0));
    let renderer = Renderer::new(
        Arc::new(many_random_spheres_with_dragons()),
        rt_weekend_camera(width, height),
        Arc::new(integrator),
        Arc::new(StratifiedSampler::default()),
        samples,
    );
    let image = renderer.render(width, height);
    image.write(&format!("{}_{}", file_name, samples));
    println!();
}
