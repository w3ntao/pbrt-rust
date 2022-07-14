use std::sync::Arc;

use crate::fundamental::obj_loader::obj_to_triangles;
use crate::fundamental::utility::*;
use crate::ray_tracing::group::Group;
use crate::ray_tracing::groups::bvh::BVH;
use crate::ray_tracing::instance::Instance;
use crate::ray_tracing::materials::diffuse_light::DiffuseLight;
use crate::ray_tracing::materials::glass::Glass;
use crate::ray_tracing::materials::lambertian::*;
use crate::ray_tracing::materials::metal::Metal;
use crate::ray_tracing::primitive::Primitive;
use crate::ray_tracing::primitives::axis_aligned_box::AxisAlignedBox;
use crate::ray_tracing::primitives::quad::Quad;
use crate::ray_tracing::primitives::sphere::Sphere;
use crate::ray_tracing::textures::solid_color::SolidColor;
use crate::ray_tracing::world::World;

const WALL_LENGTH: f32 = 555.0;

fn empty_cornell_box() -> World {
    let mut world = World::default();

    let solid_color_red = Arc::new(SolidColor::new(Color::new(0.65, 0.05, 0.05)));
    let solid_color_green = Arc::new(SolidColor::new(Color::new(0.12, 0.45, 0.15)));
    let solid_color_white = Arc::new(SolidColor::new(Color::new(0.73, 0.73, 0.73)));

    let lambertian_red = Arc::new(Lambertian::new(solid_color_red.clone()));
    let lambertian_green = Arc::new(Lambertian::new(solid_color_green.clone()));
    let lambertian_white = Arc::new(Lambertian::new(solid_color_white.clone()));

    let mut wall_left = Quad::new(Point::new(WALL_LENGTH, 0.0, 0.0), Vector3::new(0.0, WALL_LENGTH, 0.0), Vector3::new(0.0, 0.0, WALL_LENGTH));
    wall_left.set_material(lambertian_green.clone());
    let wall_left = Arc::new(wall_left);
    world.add(wall_left.clone());

    let mut wall_right = Quad::new(Point::new(0.0, 0.0, 0.0), Vector3::new(0.0, WALL_LENGTH, 0.0), Vector3::new(0.0, 0.0, WALL_LENGTH));
    wall_right.set_material(lambertian_red.clone());
    let wall_right = Arc::new(wall_right);
    world.add(wall_right.clone());

    let mut wall_back = Quad::new(Point::new(0.0, 0.0, WALL_LENGTH), Vector3::new(0.0, WALL_LENGTH, 0.0), Vector3::new(WALL_LENGTH, 0.0, 0.0));
    wall_back.set_material(lambertian_white.clone());
    let wall_back = Arc::new(wall_back);
    world.add(wall_back.clone());

    let mut wall_bottom = Quad::new(Point::new(0.0, 0.0, 0.0), Vector3::new(WALL_LENGTH, 0.0, 0.0), Vector3::new(0.0, 0.0, WALL_LENGTH));
    wall_bottom.set_material(lambertian_white.clone());
    let wall_bottom = Arc::new(wall_bottom);
    world.add(wall_bottom.clone());

    let mut wall_up = Quad::new(Point::new(0.0, WALL_LENGTH, 0.0), Vector3::new(WALL_LENGTH, 0.0, 0.0), Vector3::new(0.0, 0.0, WALL_LENGTH));
    wall_up.set_material(lambertian_white.clone());
    let wall_up = Arc::new(wall_up);
    world.add(wall_up.clone());

    world.build_index();

    return world;
}

pub fn cornell_box() -> World {
    let solid_color_white = Arc::new(SolidColor::new(Color::new(0.73, 0.73, 0.73)));
    let lambertian_white = Arc::new(Lambertian::new(solid_color_white.clone()));

    let mut world = empty_cornell_box();

    let box_big = AxisAlignedBox::new(
        Point::new(0.0, 0.0, 0.0),
        Point::new(165.0, 330.0, 165.0));
    let mut box_big = Instance::new(Arc::new(box_big));
    box_big.rotate(Vector3::new(0.0, 1.0, 0.0), PI / 12.0);
    box_big.translate(Vector3::new(265.0, 0.0, 295.0));
    box_big.set_material(lambertian_white.clone());
    let box_big = Arc::new(box_big);
    world.add(box_big.clone());

    let box_small = AxisAlignedBox::new(
        Point::new(0.0, 0.0, 0.0),
        Point::new(165.0, 165.0, 165.0));
    let mut box_small = Instance::new(Arc::new(box_small));
    box_small.rotate(Vector3::new(0.0, 1.0, 0.0), -PI / 10.0);
    box_small.translate(Vector3::new(130.0, 0.0, 65.0));
    box_small.set_material(lambertian_white.clone());
    let box_small = Arc::new(box_small);
    world.add(box_small.clone());

    let diffuse_light = DiffuseLight::new(Arc::new(SolidColor::new(Color::new(15.0, 15.0, 15.0))));
    let mut quad_light = Quad::new(Point::new(213.0, WALL_LENGTH - 1.0, 227.0), Vector3::new(130.0, 0.0, 0.0), Vector3::new(0.0, 0.0, 105.0));
    quad_light.set_material(Arc::new(diffuse_light));

    let quad_light = Arc::new(quad_light);
    world.add_light(quad_light);
    world.build_index();

    return world;
}

pub fn cornell_box_specular() -> World {
    let glass = Arc::new(Glass::new(1.5));

    let mut world = empty_cornell_box();

    let radius = 120.0;
    let mut sphere = Sphere::new(
        Point::new(190.0, radius, 190.0),
        radius);
    sphere.set_material(glass.clone());
    world.add(Arc::new(sphere));

    let diffuse_light = DiffuseLight::new(Arc::new(SolidColor::new(Color::new(15.0, 15.0, 15.0))));
    let mut quad_light = Quad::new(Point::new(213.0, WALL_LENGTH - 1.0, 227.0), Vector3::new(130.0, 0.0, 0.0), Vector3::new(0.0, 0.0, 105.0));
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

    let light_length = 120.0;
    let mut quad_light = Quad::new(Point::new((WALL_LENGTH - light_length) / 2.0, WALL_LENGTH - 1.0, (WALL_LENGTH - light_length) / 2.0), Vector3::new(light_length, 0.0, 0.0), Vector3::new(0.0, 0.0, light_length));
    quad_light.set_material(Arc::new(diffuse_light));

    let quad_light = Arc::new(quad_light);
    world.add_light(quad_light);

    let triangles = obj_to_triangles("models/dragon.obj");
    let mut dragon_model = BVH::default();
    for t in triangles {
        dragon_model.add(t);
    }

    dragon_model.build_index();
    let dragon_model = Arc::new(dragon_model);
    let mut dragon_var = Instance::new(dragon_model.clone());
    dragon_var.rotate(Vector3::new(0.0, 1.0, 0.0), 1.5 * PI);
    dragon_var.scale_by_scalar(350.0);
    dragon_var.translate(Vector3::new(WALL_LENGTH / 2.0, 98.0, WALL_LENGTH / 2.0));
    dragon_var.set_material(metal.clone());
    world.add(Arc::new(dragon_var));

    world.build_index();

    return world;
}
