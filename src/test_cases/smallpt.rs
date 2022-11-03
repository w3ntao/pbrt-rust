use crate::core::pbrt::*;

pub fn smallpt() -> World {
    let mut world = World::default();

    let solid_color_red = Arc::new(SolidColor::new(Color::new(0.75, 0.25, 0.25)));
    let solid_color_blue = Arc::new(SolidColor::new(Color::new(0.25, 0.25, 0.75)));
    let solid_color_white = Arc::new(SolidColor::new(Color::new(0.75, 0.75, 0.75)));

    let lambertian_red = Arc::new(Lambertian::new(solid_color_red.clone()));
    let lambertian_blue = Arc::new(Lambertian::new(solid_color_blue.clone()));
    let lambertian_white = Arc::new(Lambertian::new(solid_color_white.clone()));

    let wall_length = 150.0;

    let mut wall_left = Quad::new(
        Point::new(1.0, 40.8 - 0.5 * wall_length, 81.6 - 0.5 * wall_length),
        Vector3::new(0.0, wall_length, 0.0),
        Vector3::new(0.0, 0.0, wall_length),
    );
    wall_left.set_material(lambertian_red.clone());
    let wall_left = Arc::new(Primitive::new(Arc::new(wall_left)));
    world.add(wall_left.clone());

    let mut wall_right = Quad::new(
        Point::new(99.0, 40.8 - 0.5 * wall_length, 81.6 - 0.5 * wall_length),
        Vector3::new(0.0, wall_length, 0.0),
        Vector3::new(0.0, 0.0, wall_length),
    );
    wall_right.set_material(lambertian_blue.clone());
    let wall_right = Arc::new(Primitive::new(Arc::new(wall_right)));
    world.add(wall_right.clone());

    let mut wall_back = Quad::new(
        Point::new(50.0 - 0.5 * wall_length, 40.8 - 0.5 * wall_length, 0.0),
        Vector3::new(0.0, wall_length, 0.0),
        Vector3::new(wall_length, 0.0, 0.0),
    );
    wall_back.set_material(lambertian_white.clone());
    let wall_back = Arc::new(Primitive::new(Arc::new(wall_back)));
    world.add(wall_back.clone());

    let mut wall_bottom = Quad::new(
        Point::new(50.0 - 0.5 * wall_length, 0.0, 81.6 - 0.5 * wall_length),
        Vector3::new(wall_length, 0.0, 0.0),
        Vector3::new(0.0, 0.0, wall_length),
    );
    wall_bottom.set_material(lambertian_white.clone());
    let wall_bottom = Arc::new(Primitive::new(Arc::new(wall_bottom)));
    world.add(wall_bottom.clone());

    let mut wall_up = Quad::new(
        Point::new(50.0 - wall_length * 0.5, 81.6, 81.6 - wall_length * 0.5),
        Vector3::new(wall_length, 0.0, 0.0),
        Vector3::new(0.0, 0.0, wall_length),
    );

    wall_up.set_material(lambertian_white.clone());
    let wall_up = Arc::new(Primitive::new(Arc::new(wall_up)));
    world.add(wall_up.clone());

    let glass = Arc::new(Glass::new(1.5));
    let mut sphere_glass = Sphere::new(Point::new(73.0, 16.5, 78.0), 16.5);
    sphere_glass.set_material(glass.clone());
    let sphere_glass = Arc::new(Primitive::new(Arc::new(sphere_glass)));
    world.add(sphere_glass);

    let mirror = Arc::new(Mirror::new());
    let mut sphere_mirror = Sphere::new(Point::new(27.0, 16.5, 47.0), 16.5);
    sphere_mirror.set_material(mirror);
    let sphere_mirror = Arc::new(Primitive::new(Arc::new(sphere_mirror)));
    world.add(sphere_mirror);

    let light_len = 40.0;
    let diffuse_light = DiffuseLight::new(Arc::new(SolidColor::new(Color::new(12.0, 12.0, 12.0))));
    let mut quad_light = Quad::new(
        Point::new(50.0 - 0.5 * light_len, 81.6 - 0.2, 81.6 - 0.5 * light_len),
        Vector3::new(light_len, 0.0, 0.0),
        Vector3::new(0.0, 0.0, light_len),
    );
    quad_light.set_material(Arc::new(diffuse_light));

    let quad_light = Arc::new(Primitive::new(Arc::new(quad_light)));
    world.add_light(quad_light);
    world.build_index();

    return world;
}
