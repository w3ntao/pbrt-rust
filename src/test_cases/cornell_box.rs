use crate::core::pbrt::*;

const WALL_LENGTH: f32 = 555.0;

fn empty_cornell_box() -> Scene {
    let mut world = Scene::default();

    let solid_color_red = Color::new(0.65, 0.05, 0.05);
    let solid_color_green = Color::new(0.12, 0.45, 0.15);
    let solid_color_white = Color::new(0.73, 0.73, 0.73);

    let lambertian_red = Arc::new(Lambertian::new(solid_color_red));
    let lambertian_green = Arc::new(Lambertian::new(solid_color_green));
    let lambertian_white = Arc::new(Lambertian::new(solid_color_white));

    let wall_left = Quad::new(
        Point::new(WALL_LENGTH, 0.0, 0.0),
        Vector3::new(0.0, WALL_LENGTH, 0.0),
        Vector3::new(0.0, 0.0, WALL_LENGTH),
    );
    let wall_left = Arc::new(GeometricPrimitive::new(
        Arc::new(wall_left),
        lambertian_green.clone(),
    ));
    world.add(wall_left.clone());

    let wall_right = Quad::new(
        Point::new(0.0, 0.0, 0.0),
        Vector3::new(0.0, WALL_LENGTH, 0.0),
        Vector3::new(0.0, 0.0, WALL_LENGTH),
    );
    let wall_right = Arc::new(GeometricPrimitive::new(
        Arc::new(wall_right),
        lambertian_red.clone(),
    ));
    world.add(wall_right.clone());

    let wall_back = Quad::new(
        Point::new(0.0, 0.0, WALL_LENGTH),
        Vector3::new(0.0, WALL_LENGTH, 0.0),
        Vector3::new(WALL_LENGTH, 0.0, 0.0),
    );
    let wall_back = Arc::new(GeometricPrimitive::new(
        Arc::new(wall_back),
        lambertian_white.clone(),
    ));
    world.add(wall_back.clone());

    let wall_bottom = Quad::new(
        Point::new(0.0, 0.0, 0.0),
        Vector3::new(WALL_LENGTH, 0.0, 0.0),
        Vector3::new(0.0, 0.0, WALL_LENGTH),
    );
    let wall_bottom = Arc::new(GeometricPrimitive::new(
        Arc::new(wall_bottom),
        lambertian_white.clone(),
    ));
    world.add(wall_bottom.clone());

    let wall_up = Quad::new(
        Point::new(0.0, WALL_LENGTH, 0.0),
        Vector3::new(WALL_LENGTH, 0.0, 0.0),
        Vector3::new(0.0, 0.0, WALL_LENGTH),
    );
    let wall_up = Arc::new(GeometricPrimitive::new(
        Arc::new(wall_up),
        lambertian_white.clone(),
    ));
    world.add(wall_up.clone());

    world.build_index();

    return world;
}

pub fn cornell_box_camera() -> Perspective {
    let camera_center = Point::new(278.0, 278.0, -600.0);

    return Perspective::without_lens(
        camera_center,
        Vector3::new(0.0, 0.0, 1.0),
        Vector3::new(0.0, 1.0, 0.0),
        PI / 4.0,
    );
}

pub fn cornell_box() -> Scene {
    let solid_color_white = Color::new(0.73, 0.73, 0.73);
    let lambertian_white = Arc::new(Lambertian::new(solid_color_white));

    let mut world = empty_cornell_box();

    let box_big = AxisAlignedBox::new(Point::new(0.0, 0.0, 0.0), Point::new(165.0, 330.0, 165.0));
    let box_big = GeometricPrimitive::new(Arc::new(box_big), lambertian_white.clone());
    let mut box_big = TransformedPrimitive::new(Arc::new(box_big));
    box_big.rotate(Vector3::new(0.0, 1.0, 0.0), PI / 12.0);
    box_big.translate(Vector3::new(265.0, 0.0, 295.0));
    let box_big = Arc::new(box_big);
    world.add(box_big.clone());

    let box_small = AxisAlignedBox::new(Point::new(0.0, 0.0, 0.0), Point::new(165.0, 165.0, 165.0));
    let box_small = GeometricPrimitive::new(Arc::new(box_small), lambertian_white.clone());
    let mut box_small = TransformedPrimitive::new(Arc::new(box_small));
    box_small.rotate(Vector3::new(0.0, 1.0, 0.0), -PI / 10.0);
    box_small.translate(Vector3::new(130.0, 0.0, 65.0));
    let box_small = Arc::new(box_small);
    world.add(box_small.clone());

    let diffuse_light = DiffuseLight::new(Color::new(15.0, 15.0, 15.0));
    let quad_light = Quad::new(
        Point::new(213.0, WALL_LENGTH - 1.0, 227.0),
        Vector3::new(130.0, 0.0, 0.0),
        Vector3::new(0.0, 0.0, 105.0),
    );

    let quad_light = Arc::new(GeometricPrimitive::new(
        Arc::new(quad_light),
        Arc::new(diffuse_light),
    ));
    world.add_light(quad_light);
    world.build_index();

    return world;
}

pub fn cornell_box_specular() -> Scene {
    let mut world = empty_cornell_box();

    let aluminum = Arc::new(Metal::new(Color::new(0.8, 0.85, 0.88), 0.0));
    let box_big = AxisAlignedBox::new(Point::new(0.0, 0.0, 0.0), Point::new(165.0, 330.0, 165.0));
    let box_big = GeometricPrimitive::new(Arc::new(box_big), aluminum.clone());
    let mut box_big = TransformedPrimitive::new(Arc::new(box_big));

    box_big.rotate(Vector3::new(0.0, 1.0, 0.0), PI / 12.0);
    box_big.translate(Vector3::new(265.0, 0.0, 295.0));
    let box_big = Arc::new(box_big);
    world.add(box_big.clone());

    let glass = Arc::new(Glass::new(1.5));
    let radius = 90.0;
    let sphere = Sphere::new(Point::new(190.0, radius, 190.0), radius);

    let sphere = Arc::new(GeometricPrimitive::new(Arc::new(sphere), glass.clone()));
    world.add(sphere);

    let diffuse_light = DiffuseLight::new(Color::new(15.0, 15.0, 15.0));
    let quad_light = Quad::new(
        Point::new(213.0, WALL_LENGTH - 1.0, 227.0),
        Vector3::new(130.0, 0.0, 0.0),
        Vector3::new(0.0, 0.0, 105.0),
    );

    let quad_light = Arc::new(GeometricPrimitive::new(
        Arc::new(quad_light),
        Arc::new(diffuse_light),
    ));
    world.add_light(quad_light);
    world.build_index();

    return world;
}

pub fn cornell_box_metal_dragon() -> Scene {
    let mut world = empty_cornell_box();
    let metal = Arc::new(Metal::new(Color::new(0.7, 0.6, 0.5), 0.5));
    let diffuse_light = DiffuseLight::new(Color::new(10.0, 10.0, 10.0));

    let light_length = 150.0;
    let quad_light = Quad::new(
        Point::new(
            (WALL_LENGTH - light_length) / 2.0,
            WALL_LENGTH - 1.0,
            (WALL_LENGTH - light_length) / 2.0 + 150.0,
        ),
        Vector3::new(light_length, 0.0, 0.0),
        Vector3::new(0.0, 0.0, light_length),
    );

    let quad_light = Arc::new(GeometricPrimitive::new(
        Arc::new(quad_light),
        Arc::new(diffuse_light),
    ));
    world.add_light(quad_light);

    let dragon_model = Arc::new(load_dragon(metal.clone()));
    let mut dragon_instance = TransformedPrimitive::new(dragon_model.clone());
    dragon_instance.rotate(Vector3::new(0.0, 1.0, 0.0), 1.5 * PI);
    dragon_instance.scale_by_scalar(350.0);

    let bounds = dragon_instance.get_bounds();
    let center_x = (bounds.p_min.x + bounds.p_max.x) / 2.0;
    let center_z = (bounds.p_min.z + bounds.p_max.z) / 2.0;

    dragon_instance.translate(Vector3::new(
        -center_x + WALL_LENGTH / 2.0,
        -bounds.p_min.y,
        -center_z + WALL_LENGTH / 2.0,
    ));

    world.add(Arc::new(dragon_instance));

    world.build_index();

    return world;
}
