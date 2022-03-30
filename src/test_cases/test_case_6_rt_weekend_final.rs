use std::sync::Arc;

use crate::fundamental::color::Color;
use crate::fundamental::point::Point;
use crate::fundamental::utility::{get_file_name, random_in_range, random_zero_to_one};
use crate::fundamental::utility::random_color;
use crate::fundamental::vector3::Vector3;
use crate::ray_tracing::cameras::depth_of_field::DepthOfField;
use crate::ray_tracing::group::Group;
use crate::ray_tracing::groups::bvh::BVH;
use crate::ray_tracing::integrators::monte_carlo_path_trace::MonteCarloPathTrace;
use crate::ray_tracing::materials::glass::*;
use crate::ray_tracing::materials::lambertian::*;
use crate::ray_tracing::materials::metal::*;
use crate::ray_tracing::primitive::Primitive;
use crate::ray_tracing::primitives::sphere::Sphere;
use crate::ray_tracing::renderer::Renderer;
use crate::ray_tracing::world::World;

pub fn test(samples: usize) {
    let file_name = get_file_name(file!());
    println!("TESTING: {}", &file_name);
    let ppm_name = format!("{}.ppm", file_name);

    const WIDTH: usize = 1000;
    const HEIGHT: usize = 750;

    let mut scene = BVH::default();

    let material_ground = Arc::new(Lambertian::new(Color::new(0.5, 0.5, 0.5)));

    let ground_radius = 2000.0;
    let mut sphere_ground = Sphere::new(Point::new(0.0, -ground_radius, 0.0), ground_radius);
    sphere_ground.set_material(material_ground);
    scene.add(Arc::new(sphere_ground));

    for a in -11..11 {
        let a = a as f32;
        for b in -11..11 {
            let b = b as f32;
            let choose_material = random_zero_to_one();
            let center = Point::new(a + 0.9 * random_zero_to_one(), 0.2, b + 0.9 * random_zero_to_one());

            if (center - Point::new(4.0, 0.2, 0.0)).length() > 0.9 {
                let mut sphere = Sphere::new(center, 0.2);
                if choose_material < 0.4 {
                    //diffuse
                    let albedo = random_color() * random_color();
                    let material = Lambertian::new(albedo);
                    sphere.set_material(Arc::new(material));
                } else if choose_material < 0.7 {
                    // metal
                    let albedo = random_in_range(0.5, 1.0) * Color::new(1.0, 1.0, 1.0);
                    let fuzz = random_in_range(0.0, 0.5);
                    let metal = Metal::new(albedo, fuzz);
                    sphere.set_material(Arc::new(metal));
                } else {
                    //glass
                    let glass = Glass::new(1.5);
                    sphere.set_material(Arc::new(glass));
                }
                scene.add(Arc::new(sphere));
            }
        }
    }

    let lambertian = Arc::new(Lambertian::new(Color::new(0.4, 0.2, 0.1)));
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
    let integrator = MonteCarloPathTrace::new(Arc::new(world));
    let renderer = Renderer::new(Arc::new(camera), Arc::new(integrator), samples);
    let image = renderer.render(WIDTH, HEIGHT);
    image.write(&ppm_name);
    println!();
}
