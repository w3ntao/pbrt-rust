use std::sync::Arc;

use rand::random;

use crate::fundamental::color::Color;
use crate::fundamental::obj_loader::obj_to_triangles;
use crate::fundamental::point::Point;
use crate::fundamental::utility::{get_file_name, random_in_range, random_zero_to_one};
use crate::fundamental::vector3::Vector3;
use crate::ray_tracing::cameras::depth_of_field::DepthOfField;
use crate::ray_tracing::cameras::perspective::PerspectiveCamera;
use crate::ray_tracing::group::Group;
use crate::ray_tracing::groups::bvh::BVH;
use crate::ray_tracing::instance::*;
use crate::ray_tracing::integrators::monte_carlo_path_trace::MonteCarloPathTrace;
use crate::ray_tracing::integrators::ray_casting::RayCastingIntegrator;
use crate::ray_tracing::materials::glass::*;
use crate::ray_tracing::materials::lambertian::*;
use crate::ray_tracing::materials::metal::*;
use crate::ray_tracing::materials::mirror::*;
use crate::ray_tracing::primitive::Primitive;
use crate::ray_tracing::primitives::hollow_sphere::HollowSphere;
use crate::ray_tracing::primitives::sphere::Sphere;
use crate::ray_tracing::ray::Ray;
use crate::ray_tracing::renderer::Renderer;
use crate::ray_tracing::world::World;

fn random_color() -> Color {
    return Color::new(random_zero_to_one(), random_zero_to_one(), random_zero_to_one());
}

pub fn test(samples: i32) {
    let file_name = get_file_name(file!());
    println!("TESTING: {}", &file_name);
    let ppm_name = format!("{}.ppm", file_name);

    const WIDTH: usize = 1000;
    const HEIGHT: usize = 750;

    let mut scene = BVH::default();

    let material_ground = Arc::new(Lambertian::new(Color::new(0.5, 0.5, 0.5)));

    let ground_radius = 10000.0;
    let mut sphere_ground = Sphere::new(Point::new(0.0, -ground_radius, 0.0), ground_radius);
    sphere_ground.set_material(material_ground);
    let ground = Arc::new(sphere_ground);
    scene.add(ground.clone());

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
                //scene.add(Arc::new(sphere));
            }
        }
    }

    let glass = Arc::new(Glass::new(1.5));
    let metal = Arc::new(Metal { albedo: Color::new(0.7, 0.6, 0.5), fuzz: 0.0 });

    //let triangles = obj_to_triangles("models/lucy_winged_victory.obj");
    let triangles = obj_to_triangles("models/dragon.obj");
    let mut lucy_bvh = BVH::default();
    for t in triangles {
        lucy_bvh.add(t);
    }
    lucy_bvh.build_index();
    let lucy_bvh_arc = Arc::new(lucy_bvh);

    let scale = 4.0;

    let mut lucy_instance_0 = Instance::new(lucy_bvh_arc.clone());
    lucy_instance_0.set_material(metal.clone());
    lucy_instance_0.scale_by_scalar(scale);
    //lucy_instance_0.translate(Vector3::new(-4.0, 0.0, 0.0));
    //lucy_instance_0.translate(Vector3::new(0.0, -1.0, 0.0));
    //scene.add(Arc::new(lucy_instance_0));

    let mut lucy_instance_1 = Instance::new(lucy_bvh_arc.clone());
    lucy_instance_1.set_material(metal.clone());
    lucy_instance_1.scale_by_scalar(scale);
    lucy_instance_1.translate(Vector3::new(0.0, 0.0, 0.0));
    //scene.add(Arc::new(lucy_instance_1));

    let mut lucy_instance_2 = Instance::new(lucy_bvh_arc.clone());
    lucy_instance_2.set_material(metal.clone());
    lucy_instance_2.scale_by_scalar(scale);
    lucy_instance_2.translate(Vector3::new(4.0, 0.0, 0.0));
    //scene.add(Arc::new(lucy_instance_2));

    let material0 = Lambertian::new(Color::new(0.4, 0.2, 0.1));
    let mut sphere0 = Sphere::new(Point::new(-4.0, 1.0, 0.0), 1.0);
    sphere0.set_material(Arc::new(material0));
    //scene.add(Arc::new(sphere0));

    let sphere1_center = Point::new(0.0, 1.0, 0.0);
    let mut sphere1 = Sphere::new(sphere1_center, 1.0);
    sphere1.set_material(glass.clone());
    //scene.add(Arc::new(sphere1));

    let sphere2_center = Point::new(4.0, 1.0, 0.0);
    let mut sphere2 = Sphere::new(sphere2_center, 1.0);
    sphere2.set_material(metal.clone());
    //scene.add(Arc::new(sphere2));

    scene.build_index();

    let camera_center = Point::new(13.0, 2.0, 3.0);
    let look_at = Point::new(0.0, 0.0, 0.0);
    let direction = look_at - camera_center;

    let camera = PerspectiveCamera::new(
        camera_center,
        direction,
        Vector3::new(0.0, 1.0, 0.0),
        std::f32::consts::PI / 8.0,
        std::f32::consts::PI / 6.0);

    let world = World::new(Arc::new(scene));
    let integrator = MonteCarloPathTrace::new(Arc::new(world));
    let renderer = Renderer::new(Arc::new(camera), Arc::new(integrator), samples);
    let image = renderer.render(WIDTH, HEIGHT);
    image.write(&ppm_name);
    println!();
}
