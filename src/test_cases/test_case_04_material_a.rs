use crate::cameras::perspective::Perspective;
use crate::core::pbrt::*;
use crate::integrators::monte_carlo_path_trace::MonteCarloPathTrace;
use crate::materials::glass::*;
use crate::materials::lambertian::*;
use crate::materials::metal::*;
use crate::materials::mirror::*;
use crate::primitives::sphere::Sphere;
use crate::textures::solid_color::SolidColor;
use crate::utility::get_file_name;
use std::sync::Arc;

#[allow(dead_code)]
pub fn test(width: usize, height: usize, samples: u32) {
    let samples = ((samples as f32).sqrt() as u32).pow(2);
    let file_name = get_file_name(file!());
    let ppm_name = format!("{}_{}.ppm", file_name, samples);
    println!("TESTING: {} for {} samples per pixel", &file_name, samples);

    let material_ground = Arc::new(Lambertian::new(Arc::new(SolidColor::new(Color::new(
        0.8, 0.8, 0.0,
    )))));
    let metal = Arc::new(Metal::new(Color::new(0.8, 0.6, 0.2), 0.4));
    let mirror = Arc::new(Mirror::new());
    let glass = Arc::new(Glass::new(1.5));

    let mut sphere_ground = Sphere::new(Point::new(0.0, -100.5, -1.0), 100.0);
    sphere_ground.set_material(material_ground);

    let mut sphere_center = Sphere::new(Point::new(0.0, 0.0, -1.0), 0.5);
    sphere_center.set_material(glass);

    let mut sphere_left = Sphere::new(Point::new(-1.0, 0.0, -1.0), 0.5);
    sphere_left.set_material(mirror);

    let mut sphere_right = Sphere::new(Point::new(1.0, 0.0, -1.0), 0.5);
    sphere_right.set_material(metal);

    let mut world = World::default();
    world.add(Arc::new(sphere_ground));
    world.add(Arc::new(sphere_left));
    world.add(Arc::new(sphere_center));
    world.add(Arc::new(sphere_right));
    world.build_index();

    let camera = Perspective::new(
        Point::new(0.0, 0.0, 5.0),
        Vector3::new(0.0, 0.0, -1.0),
        Vector3::new(0.0, 1.0, 0.0),
        PI / 6.0,
        (height as f32) / (width as f32),
    );

    let integrator = MonteCarloPathTrace::new(Arc::new(world), Color::new(0.7, 0.8, 1.0));
    let renderer = Renderer::new(Arc::new(camera), Arc::new(integrator), samples);
    let image = renderer.render(width, height);
    image.write(&ppm_name);
    println!();
}
