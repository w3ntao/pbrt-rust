use crate::cameras::perspective::Perspective;
use crate::core::instance::Instance;
use crate::core::primitive::Primitive;
use crate::core::world::World;
use crate::fundamental::utility::*;
use crate::materials::diffuse_light::DiffuseLight;
use crate::materials::glass::Glass;
use crate::materials::lambertian::*;
use crate::materials::metal::Metal;
use crate::primitives::axis_aligned_box::AxisAlignedBox;
use crate::primitives::quad::Quad;
use crate::primitives::sphere::Sphere;
use crate::textures::solid_color::SolidColor;
use crate::utility::load_dragon;
use std::sync::Arc;

const WALL_LENGTH: f32 = 555.0;

fn empty_cornell_box() -> World {
    let mut world = World::default();

    let solid_color_red = Arc::new(SolidColor::new(Color::new(0.65, 0.05, 0.05)));
    let solid_color_green = Arc::new(SolidColor::new(Color::new(0.12, 0.45, 0.15)));
    let solid_color_white = Arc::new(SolidColor::new(Color::new(0.73, 0.73, 0.73)));

    let lambertian_red = Arc::new(Lambertian::new(solid_color_red.clone()));
    let lambertian_green = Arc::new(Lambertian::new(solid_color_green.clone()));
    let lambertian_white = Arc::new(Lambertian::new(solid_color_white.clone()));

    let mut wall_left = Quad::new(
        Point::new(WALL_LENGTH, 0.0, 0.0),
        Vector3::new(0.0, WALL_LENGTH, 0.0),
        Vector3::new(0.0, 0.0, WALL_LENGTH),
    );
    wall_left.set_material(lambertian_green.clone());
    let wall_left = Arc::new(wall_left);
    world.add(wall_left.clone());

    let mut wall_right = Quad::new(
        Point::new(0.0, 0.0, 0.0),
        Vector3::new(0.0, WALL_LENGTH, 0.0),
        Vector3::new(0.0, 0.0, WALL_LENGTH),
    );
    wall_right.set_material(lambertian_red.clone());
    let wall_right = Arc::new(wall_right);
    world.add(wall_right.clone());

    let mut wall_back = Quad::new(
        Point::new(0.0, 0.0, WALL_LENGTH),
        Vector3::new(0.0, WALL_LENGTH, 0.0),
        Vector3::new(WALL_LENGTH, 0.0, 0.0),
    );
    wall_back.set_material(lambertian_white.clone());
    let wall_back = Arc::new(wall_back);
    world.add(wall_back.clone());

    let mut wall_bottom = Quad::new(
        Point::new(0.0, 0.0, 0.0),
        Vector3::new(WALL_LENGTH, 0.0, 0.0),
        Vector3::new(0.0, 0.0, WALL_LENGTH),
    );
    wall_bottom.set_material(lambertian_white.clone());
    let wall_bottom = Arc::new(wall_bottom);
    world.add(wall_bottom.clone());

    let mut wall_up = Quad::new(
        Point::new(0.0, WALL_LENGTH, 0.0),
        Vector3::new(WALL_LENGTH, 0.0, 0.0),
        Vector3::new(0.0, 0.0, WALL_LENGTH),
    );
    wall_up.set_material(lambertian_white.clone());
    let wall_up = Arc::new(wall_up);
    world.add(wall_up.clone());

    world.build_index();

    return world;
}

pub fn cornell_box_camera(width: usize, height: usize) -> Perspective {
    let camera_center = Point::new(278.0, 278.0, -600.0);

    return Perspective::new(
        camera_center,
        Vector3::new(0.0, 0.0, 1.0),
        Vector3::new(0.0, 1.0, 0.0),
        PI / 4.0,
        (height as f32) / (width as f32),
    );
}

pub fn cornell_box() -> World {
    let solid_color_white = Arc::new(SolidColor::new(Color::new(0.73, 0.73, 0.73)));
    let lambertian_white = Arc::new(Lambertian::new(solid_color_white.clone()));

    let mut world = empty_cornell_box();

    let box_big = AxisAlignedBox::new(Point::new(0.0, 0.0, 0.0), Point::new(165.0, 330.0, 165.0));
    let mut box_big = Instance::new(Arc::new(box_big));
    box_big.rotate(Vector3::new(0.0, 1.0, 0.0), PI / 12.0);
    box_big.translate(Vector3::new(265.0, 0.0, 295.0));
    box_big.set_material(lambertian_white.clone());
    let box_big = Arc::new(box_big);
    world.add(box_big.clone());

    let box_small = AxisAlignedBox::new(Point::new(0.0, 0.0, 0.0), Point::new(165.0, 165.0, 165.0));
    let mut box_small = Instance::new(Arc::new(box_small));
    box_small.rotate(Vector3::new(0.0, 1.0, 0.0), -PI / 10.0);
    box_small.translate(Vector3::new(130.0, 0.0, 65.0));
    box_small.set_material(lambertian_white.clone());
    let box_small = Arc::new(box_small);
    world.add(box_small.clone());

    let diffuse_light = DiffuseLight::new(Arc::new(SolidColor::new(Color::new(15.0, 15.0, 15.0))));
    let mut quad_light = Quad::new(
        Point::new(213.0, WALL_LENGTH - 1.0, 227.0),
        Vector3::new(130.0, 0.0, 0.0),
        Vector3::new(0.0, 0.0, 105.0),
    );
    quad_light.set_material(Arc::new(diffuse_light));

    let quad_light = Arc::new(quad_light);
    world.add_light(quad_light);
    world.build_index();

    return world;
}

pub fn cornell_box_specular() -> World {
    let mut world = empty_cornell_box();

    let aluminum = Arc::new(Metal::new(Color::new(0.8, 0.85, 0.88), 0.0));
    let box_big = AxisAlignedBox::new(Point::new(0.0, 0.0, 0.0), Point::new(165.0, 330.0, 165.0));
    let mut box_big = Instance::new(Arc::new(box_big));
    box_big.rotate(Vector3::new(0.0, 1.0, 0.0), PI / 12.0);
    box_big.translate(Vector3::new(265.0, 0.0, 295.0));
    box_big.set_material(aluminum.clone());
    let box_big = Arc::new(box_big);
    world.add(box_big.clone());

    let glass = Arc::new(Glass::new(1.5));
    let radius = 90.0;
    let mut sphere = Sphere::new(Point::new(190.0, radius, 190.0), radius);
    sphere.set_material(glass.clone());
    world.add(Arc::new(sphere));

    let diffuse_light = DiffuseLight::new(Arc::new(SolidColor::new(Color::new(15.0, 15.0, 15.0))));
    let mut quad_light = Quad::new(
        Point::new(213.0, WALL_LENGTH - 1.0, 227.0),
        Vector3::new(130.0, 0.0, 0.0),
        Vector3::new(0.0, 0.0, 105.0),
    );
    quad_light.set_material(Arc::new(diffuse_light));

    let quad_light = Arc::new(quad_light);
    world.add_light(quad_light);
    world.build_index();

    return world;
}

pub fn cornell_box_metal_dragon() -> World {
    let mut world = empty_cornell_box();
    let metal = Arc::new(Metal::new(Color::new(0.7, 0.6, 0.5), 0.5));

    let diffuse_light = DiffuseLight::new(Arc::new(SolidColor::new(Color::new(10.0, 10.0, 10.0))));

    let light_length = 150.0;
    let mut quad_light = Quad::new(
        Point::new(
            (WALL_LENGTH - light_length) / 2.0,
            WALL_LENGTH - 1.0,
            (WALL_LENGTH - light_length) / 2.0 + 150.0,
        ),
        Vector3::new(light_length, 0.0, 0.0),
        Vector3::new(0.0, 0.0, light_length),
    );
    quad_light.set_material(Arc::new(diffuse_light));

    let quad_light = Arc::new(quad_light);
    world.add_light(quad_light);

    let dragon_model = Arc::new(load_dragon());
    let mut dragon_instance = Instance::new(dragon_model.clone());
    dragon_instance.rotate(Vector3::new(0.0, 1.0, 0.0), 1.5 * PI);
    dragon_instance.scale_by_scalar(350.0);

    let bounds = dragon_instance.get_bounds();
    let center_x = (bounds.min.x + bounds.max.x) / 2.0;
    let center_z = (bounds.min.z + bounds.max.z) / 2.0;

    dragon_instance.translate(Vector3::new(
        -center_x + WALL_LENGTH / 2.0,
        -bounds.min.y,
        -center_z + WALL_LENGTH / 2.0,
    ));
    dragon_instance.set_material(metal.clone());
    world.add(Arc::new(dragon_instance));

    world.build_index();

    return world;
}
