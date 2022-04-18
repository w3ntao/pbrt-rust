use std::sync::Arc;

use crate::fundamental::color::Color;
use crate::fundamental::point::Point;
use crate::fundamental::utility::get_file_name;
use crate::fundamental::vector3::Vector3;
use crate::ray_tracing::cameras::depth_of_field::DepthOfField;
use crate::ray_tracing::group::Group;
use crate::ray_tracing::integrators::monte_carlo_path_trace::MonteCarloPathTrace;
use crate::ray_tracing::materials::glass::*;
use crate::ray_tracing::materials::lambertian::*;
use crate::ray_tracing::materials::metal::*;
use crate::ray_tracing::primitive::Primitive;
use crate::ray_tracing::primitives::sphere::Sphere;
use crate::ray_tracing::renderer::Renderer;
use crate::ray_tracing::textures::checker_texture::CheckerTexture;
use crate::ray_tracing::textures::solid_color::*;
use crate::ray_tracing::world::World;
use crate::test_case_6_rt_weekend_final::many_random_spheres;

pub fn test(samples: u32) {
    let file_name = get_file_name(file!());
    println!("TESTING: {}", &file_name);
    let ppm_name = format!("{}.ppm", file_name);

    const WIDTH: usize = 1000;
    const HEIGHT: usize = 750;

    let mut scene = many_random_spheres();

    let solid_green = Arc::new(SolidColor::new(Color::new(0.2, 0.3, 0.1)));
    let solid_white = Arc::new(SolidColor::new(Color::new(0.9, 0.9, 0.9)));

    let checker = Arc::new(CheckerTexture::new(solid_green.clone(), solid_white.clone()));
    let material_ground = Arc::new(Lambertian::new(checker.clone()));

    let ground_radius = 1000.0;
    let mut sphere_ground = Sphere::new(Point::new(0.0, -ground_radius, 0.0), ground_radius);
    sphere_ground.set_material(material_ground);
    scene.add(Arc::new(sphere_ground));

    let texture_lambertian = Arc::new(SolidColor::new(Color::new(0.4, 0.2, 0.1)));
    let lambertian = Arc::new(Lambertian::new(texture_lambertian.clone()));

    let glass = Arc::new(Glass::new(1.5));
    let metal = Arc::new(Metal::new(Color::new(0.7, 0.6, 0.5), 0.0));

    let mut sphere_far = Sphere::new(Point::new(-4.0, 1.0, 0.0), 1.0);
    sphere_far.set_material(lambertian.clone());
    scene.add(Arc::new(sphere_far));

    let mut sphere_middle = Sphere::new(Point::new(0.0, 1.0, 0.0), 1.0);
    sphere_middle.set_material(glass.clone());
    scene.add(Arc::new(sphere_middle));

    let center_sphere_close = Point::new(4.0, 1.0, 0.0);
    let mut sphere_close = Sphere::new(center_sphere_close, 1.0);
    sphere_close.set_material(metal.clone());
    scene.add(Arc::new(sphere_close));

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
        0.2, (camera_center - center_sphere_close).length());

    let world = World::new(Arc::new(scene));
    let integrator = MonteCarloPathTrace::new(Arc::new(world), Color::new(0.7, 0.8, 1.0));
    let renderer = Renderer::new(Arc::new(camera), Arc::new(integrator), samples);
    let image = renderer.render(WIDTH, HEIGHT);
    image.write(&ppm_name);
    println!();
}
