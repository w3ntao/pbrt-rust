use std::sync::Arc;

use crate::fundamental::color::Color;
use crate::fundamental::obj_loader::obj_to_triangles;
use crate::fundamental::point::Point;
use crate::fundamental::utility::get_file_name;
use crate::fundamental::vector3::Vector3;
use crate::ray_tracing::cameras::depth_of_field::DepthOfField;
use crate::ray_tracing::group::Group;
use crate::ray_tracing::groups::bvh::BVH;
use crate::ray_tracing::instance::*;
use crate::ray_tracing::integrators::monte_carlo_path_trace::MonteCarloPathTrace;
use crate::ray_tracing::materials::glass::*;
use crate::ray_tracing::materials::lambertian::*;
use crate::ray_tracing::materials::metal::*;
use crate::ray_tracing::primitive::Primitive;
use crate::ray_tracing::primitives::sphere::Sphere;
use crate::ray_tracing::renderer::Renderer;
use crate::ray_tracing::textures::solid_color::SolidColor;
use crate::ray_tracing::world::World;
use crate::test_case_6_rt_weekend_final::many_random_spheres;

pub fn test(samples: u32) {
    let file_name = get_file_name(file!());
    println!("TESTING: {}", &file_name);
    let ppm_name = format!("{}.ppm", file_name);

    const WIDTH: usize = 1000;
    const HEIGHT: usize = 750;

    let mut scene = many_random_spheres();
    let solid_color_ground = Arc::new(SolidColor::new(Color::new(0.5, 0.5, 0.5)));
    let material_ground = Arc::new(Lambertian::new(solid_color_ground.clone()));

    let ground_radius = 2000.0;
    let mut sphere_ground = Sphere::new(Point::new(0.0, -ground_radius, 0.0), ground_radius);
    sphere_ground.set_material(material_ground);
    let ground = Arc::new(sphere_ground);
    scene.add(ground.clone());

    let texture_lambertian = Arc::new(SolidColor::new(Color::new(0.4, 0.2, 0.1)));
    let lambertian = Arc::new(Lambertian::new(texture_lambertian.clone()));
    let glass = Arc::new(Glass::new(1.5));
    let metal = Arc::new(Metal::new(Color::new(0.7, 0.6, 0.5), 0.0));

    let mut dragon_bvh = BVH::default();
    for t in obj_to_triangles("models/dragon.obj") {
        dragon_bvh.add(t);
    }
    dragon_bvh.build_index();

    let mut scaled_dragon = Instance::new(Arc::new(dragon_bvh));
    scaled_dragon.rotate(Vector3::new(0.0, 1.0, 0.0), std::f32::consts::PI);
    scaled_dragon.translate(Vector3::new(0.0, 0.7, 0.0));
    scaled_dragon.scale_by_scalar(2.5);
    let dragon_instance = Arc::new(scaled_dragon);

    let mut dragon_instance_0 = Instance::new(dragon_instance.clone());
    dragon_instance_0.set_material(lambertian.clone());
    dragon_instance_0.translate(Vector3::new(-4.0, 0.0, 0.0));
    scene.add(Arc::new(dragon_instance_0));

    let mut dragon_instance_1 = Instance::new(dragon_instance.clone());
    dragon_instance_1.set_material(glass.clone());
    dragon_instance_1.translate(Vector3::new(0.0, 0.0, 0.0));
    scene.add(Arc::new(dragon_instance_1));

    let mut dragon_instance_2 = Instance::new(dragon_instance.clone());
    dragon_instance_2.set_material(metal.clone());
    dragon_instance_2.translate(Vector3::new(4.0, 0.0, 0.0));
    scene.add(Arc::new(dragon_instance_2));

    let middle_dragon_center = Point::new(0.0, 1.0, 0.0);

    scene.build_index();

    let camera_center = Point::new(13.0, 2.0, 3.0);
    let look_at = Point::new(0.0, 0.0, 0.0);
    let direction = look_at - camera_center;

    let camera = DepthOfField::new(
        camera_center,
        direction,
        Vector3::new(0.0, 1.0, 0.0),
        std::f32::consts::PI / 8.0,
        std::f32::consts::PI / 6.0,
        0.15, (camera_center - middle_dragon_center).length(),
    );

    let world = World::new(Arc::new(scene));
    let integrator = MonteCarloPathTrace::new(Arc::new(world), Color::new(0.7, 0.8, 1.0));
    let renderer = Renderer::new(Arc::new(camera), Arc::new(integrator), samples);
    let image = renderer.render(WIDTH, HEIGHT);
    image.write(&ppm_name);
    println!();
}
