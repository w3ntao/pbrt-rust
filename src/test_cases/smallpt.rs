use crate::core::pbrt::*;

pub fn smallpt() -> World {
    let mut world = World::default();

    let solid_color_red = Color::new(0.75, 0.25, 0.25);
    let solid_color_blue = Color::new(0.25, 0.25, 0.75);
    let solid_color_white = Color::new(0.75, 0.75, 0.75);

    let lambertian_red = Arc::new(Lambertian::new(solid_color_red));
    let lambertian_blue = Arc::new(Lambertian::new(solid_color_blue));
    let lambertian_white = Arc::new(Lambertian::new(solid_color_white));

    let wall_length = 150.0;

    let wall_left = Quad::new(
        Point::new(1.0, 40.8 - 0.5 * wall_length, 81.6 - 0.5 * wall_length),
        Vector3::new(0.0, wall_length, 0.0),
        Vector3::new(0.0, 0.0, wall_length),
    );
    let wall_left = Arc::new(GeometricPrimitive::new(
        Arc::new(wall_left),
        lambertian_red.clone(),
    ));
    world.add(wall_left.clone());

    let wall_right = Quad::new(
        Point::new(99.0, 40.8 - 0.5 * wall_length, 81.6 - 0.5 * wall_length),
        Vector3::new(0.0, wall_length, 0.0),
        Vector3::new(0.0, 0.0, wall_length),
    );
    let wall_right = Arc::new(GeometricPrimitive::new(
        Arc::new(wall_right),
        lambertian_blue.clone(),
    ));
    world.add(wall_right.clone());

    let wall_back = Quad::new(
        Point::new(50.0 - 0.5 * wall_length, 40.8 - 0.5 * wall_length, 0.0),
        Vector3::new(0.0, wall_length, 0.0),
        Vector3::new(wall_length, 0.0, 0.0),
    );
    let wall_back = Arc::new(GeometricPrimitive::new(
        Arc::new(wall_back),
        lambertian_white.clone(),
    ));
    world.add(wall_back.clone());

    let wall_bottom = Quad::new(
        Point::new(50.0 - 0.5 * wall_length, 0.0, 81.6 - 0.5 * wall_length),
        Vector3::new(wall_length, 0.0, 0.0),
        Vector3::new(0.0, 0.0, wall_length),
    );
    let wall_bottom = Arc::new(GeometricPrimitive::new(
        Arc::new(wall_bottom),
        lambertian_white.clone(),
    ));
    world.add(wall_bottom.clone());

    let wall_up = Quad::new(
        Point::new(50.0 - wall_length * 0.5, 81.6, 81.6 - wall_length * 0.5),
        Vector3::new(wall_length, 0.0, 0.0),
        Vector3::new(0.0, 0.0, wall_length),
    );

    let wall_up = Arc::new(GeometricPrimitive::new(
        Arc::new(wall_up),
        lambertian_white.clone(),
    ));
    world.add(wall_up.clone());

    let glass = Arc::new(Glass::new(1.5));
    let sphere_glass = Sphere::new(Point::new(73.0, 16.5, 78.0), 16.5);
    let sphere_glass = Arc::new(GeometricPrimitive::new(
        Arc::new(sphere_glass),
        glass.clone(),
    ));
    world.add(sphere_glass);

    let mirror = Arc::new(Mirror::new());
    let sphere_mirror = Sphere::new(Point::new(27.0, 16.5, 47.0), 16.5);
    let sphere_mirror = Arc::new(GeometricPrimitive::new(Arc::new(sphere_mirror), mirror));
    world.add(sphere_mirror);

    let light_len = 40.0;
    let diffuse_light = DiffuseLight::new(Color::new(12.0, 12.0, 12.0));
    let quad_light = Quad::new(
        Point::new(50.0 - 0.5 * light_len, 81.6 - 0.2, 81.6 - 0.5 * light_len),
        Vector3::new(light_len, 0.0, 0.0),
        Vector3::new(0.0, 0.0, light_len),
    );

    let quad_light = Arc::new(GeometricPrimitive::new(
        Arc::new(quad_light),
        Arc::new(diffuse_light),
    ));
    world.add_light(quad_light);
    world.build_index();

    return world;
}
