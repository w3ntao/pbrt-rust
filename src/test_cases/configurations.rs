use crate::core::pbrt::*;
use crate::test_cases::cornell_box::*;
use crate::test_cases::smallpt::*;

pub fn create_dragon_in_the_air(width: usize, height: usize) -> Arc<Configuration> {
    let mut world = Scene::default();
    let glass = Arc::new(Glass::new(1.5));
    let dragon_model = Arc::new(load_dragon(glass.clone()));
    let dragon_instance = TransformedPrimitive::new(dragon_model.clone());
    world.add(Arc::new(dragon_instance));
    world.build_index();
    let world = Arc::new(world);

    let camera = Perspective::without_lens(
        Point::new(-2.9, 0.0, 0.0),
        Vector3::new(1.0, 0.0, 0.0),
        Vector3::new(0.0, 1.0, 0.0),
        PI / 6.0,
        (height as f32) / (width as f32),
    );

    let integrator = DebuggerRayCastingDotNormal::default();

    return Arc::new(Configuration::new(
        world.clone(),
        Arc::new(camera),
        Arc::new(integrator),
        Arc::new(StratifiedSampler::default()),
    ));
}

pub fn create_transformed_dragon_in_the_air(width: usize, height: usize) -> Arc<Configuration> {
    pub fn far_camera(width: usize, height: usize) -> Perspective {
        let offset = 1200.0;

        let camera = Perspective::without_lens(
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

    let configuration = Configuration::new(
        Arc::new(transformed_dragon()),
        Arc::new(far_camera(width, height)),
        Arc::new(DebuggerRayCastingDotNormal::default()),
        Arc::new(StratifiedSampler::default()),
    );

    return Arc::new(configuration);
}

pub fn create_bvh_many_dragons(width: usize, height: usize) -> Arc<Configuration> {
    let glass = Arc::new(Glass::new(1.5));
    let dragon_model = Arc::new(load_dragon(glass.clone()));

    let num = 2000;
    let ratio: f32 = 20000.0;
    let delta = 0.2;

    let mut scene = Scene::default();
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

        scene.add(Arc::new(dragon_instance));
    }
    scene.build_index();
    let scene = Arc::new(scene);

    let origin = Point::new(-7.0, 4.0, 0.0);
    let focus = Point::new(0.0, 0.0, 0.0);
    let look_at = focus - origin;

    let camera = Perspective::without_lens(
        origin + (-ratio * look_at * 10.0),
        look_at,
        Vector3::new(0.0, 1.0, 0.0),
        PI / 6.0,
        (height as f32) / (width as f32),
    );

    let configuration = Configuration::new(
        scene.clone(),
        Arc::new(camera),
        Arc::new(DebuggerRayCastingDotNormal::default()),
        Arc::new(StratifiedSampler::default()),
    );

    return Arc::new(configuration);
}

pub fn create_cornell_box_dragon(width: usize, height: usize) -> Arc<Configuration> {
    let configuration = Configuration::new(
        Arc::new(cornell_box_metal_dragon()),
        Arc::new(cornell_box_camera(width, height)),
        Arc::new(PathTrace::default()),
        Arc::new(StratifiedSampler::default()),
    );

    return Arc::new(configuration);
}

pub fn create_rt_weekend(width: usize, height: usize) -> Arc<Configuration> {
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

        let mut scaled_dragon =
            TransformedPrimitive::new(Arc::new(load_dragon(lambertian.clone())));
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
            0.1,
            (camera_center - middle_dragon_center).length(),
        );

        return Arc::new(camera);
    }

    let integrator = PathTrace::new(Color::new(0.7, 0.8, 1.0));
    let configuration = Configuration::new(
        Arc::new(many_random_spheres_with_dragons()),
        rt_weekend_camera(width, height),
        Arc::new(integrator),
        Arc::new(StratifiedSampler::default()),
    );
    return Arc::new(configuration);
}

pub fn create_cornell_box_lambertian(width: usize, height: usize) -> Arc<Configuration> {
    let configuration = Configuration::new(
        Arc::new(cornell_box()),
        Arc::new(cornell_box_camera(width, height)),
        Arc::new(PathTrace::default()),
        Arc::new(StratifiedSampler::default()),
    );
    return Arc::new(configuration);
}

pub fn create_cornell_box_specular(width: usize, height: usize) -> Arc<Configuration> {
    let configuration = Configuration::new(
        Arc::new(cornell_box_specular()),
        Arc::new(cornell_box_camera(width, height)),
        Arc::new(PathTrace::default()),
        Arc::new(StratifiedSampler::default()),
    );
    return Arc::new(configuration);
}

pub fn create_smallpt(width: usize, height: usize) -> Arc<Configuration> {
    let camera_center = Point::new(50.0, 52.0, 275.6);
    let direction = Vector3::new(0.0, -0.042612, -1.0);

    let camera = Perspective::without_lens(
        camera_center,
        direction,
        Vector3::new(0.0, 1.0, 0.0),
        PI / 4.3,
        (height as f32) / (width as f32),
    );

    let configuration = Configuration::new(
        Arc::new(smallpt()),
        Arc::new(camera),
        Arc::new(PathTrace::default()),
        Arc::new(StratifiedSampler::default()),
    );
    return Arc::new(configuration);
}
