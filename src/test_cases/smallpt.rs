use std::sync::Arc;

use crate::fundamental::utility::*;
use crate::ray_tracing::materials::diffuse_light::DiffuseLight;
use crate::ray_tracing::materials::lambertian::*;
use crate::ray_tracing::primitive::Primitive;
use crate::ray_tracing::primitives::quad::Quad;
use crate::ray_tracing::textures::solid_color::SolidColor;
use crate::ray_tracing::world::World;

const WALL_LENGTH: f32 = 1000.0;

pub fn smallpt() -> World {
    let mut world = World::default();

    let solid_color_red = Arc::new(SolidColor::new(Color::new(0.75, 0.25, 0.25)));
    let solid_color_blue = Arc::new(SolidColor::new(Color::new(0.25, 0.25, 0.75)));
    let solid_color_white = Arc::new(SolidColor::new(Color::new(0.75, 0.75, 0.75)));

    let lambertian_red = Arc::new(Lambertian::new(solid_color_red.clone()));
    let lambertian_blue = Arc::new(Lambertian::new(solid_color_blue.clone()));
    let lambertian_white = Arc::new(Lambertian::new(solid_color_white.clone()));

    let mut wall_left = Quad::new(Point::new(1.0, -200.0, -200.0), Vector3::new(0.0, WALL_LENGTH, 0.0), Vector3::new(0.0, 0.0, WALL_LENGTH));
    wall_left.set_material(lambertian_red.clone());
    let wall_left = Arc::new(wall_left);
    world.add(wall_left.clone());

    let mut wall_right = Quad::new(Point::new(99.0, -200.0, -200.0), Vector3::new(0.0, WALL_LENGTH, 0.0), Vector3::new(0.0, 0.0, WALL_LENGTH));    wall_right.set_material(lambertian_blue.clone());
    let wall_right = Arc::new(wall_right);
    world.add(wall_right.clone());

    let mut wall_back = Quad::new(Point::new(0.0, 0.0, WALL_LENGTH), Vector3::new(0.0, WALL_LENGTH, 0.0), Vector3::new(WALL_LENGTH, 0.0, 0.0));
    wall_back.set_material(lambertian_white.clone());
    let wall_back = Arc::new(wall_back);
    //world.add(wall_back.clone());

    let mut wall_bottom = Quad::new(Point::new(0.0, 0.0, 0.0), Vector3::new(WALL_LENGTH, 0.0, 0.0), Vector3::new(0.0, 0.0, WALL_LENGTH));
    wall_bottom.set_material(lambertian_white.clone());
    let wall_bottom = Arc::new(wall_bottom);
    //world.add(wall_bottom.clone());

    let mut wall_up = Quad::new(Point::new(0.0, WALL_LENGTH, 0.0), Vector3::new(WALL_LENGTH, 0.0, 0.0), Vector3::new(0.0, 0.0, WALL_LENGTH));
    wall_up.set_material(lambertian_white.clone());
    let wall_up = Arc::new(wall_up);
    //world.add(wall_up.clone());

    let diffuse_light = DiffuseLight::new(Arc::new(SolidColor::new(Color::new(15.0, 15.0, 15.0))));
    let mut quad_light = Quad::new(Point::new(213.0, WALL_LENGTH - 1.0, 227.0), Vector3::new(130.0, 0.0, 0.0), Vector3::new(0.0, 0.0, 105.0));
    //quad_light.set_material(Arc::new(diffuse_light));

    let quad_light = Arc::new(quad_light);
    //world.add_light(quad_light);
    world.build_index();

    return world;
}