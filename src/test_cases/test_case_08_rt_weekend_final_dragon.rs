use crate::cameras::depth_of_field::DepthOfField;
use crate::core::interfaces::*;
use crate::integrators::monte_carlo_path_trace::MonteCarloPathTrace;
use crate::materials::glass::*;
use crate::materials::lambertian::*;
use crate::materials::metal::*;
use crate::primitives::sphere::Sphere;
use crate::test_case_07_rt_weekend_final::many_random_spheres;
use crate::textures::solid_color::SolidColor;
use crate::utility::{get_file_name, load_dragon};
use std::sync::Arc;

#[allow(dead_code)]
pub fn test(width: usize, height: usize, samples: u32) {
    let samples = ((samples as f32).sqrt() as u32).pow(2);
    let file_name = get_file_name(file!());
    let ppm_name = format!("{}_{}.ppm", file_name, samples);
    println!("TESTING: {} for {} samples per pixel", &file_name, samples);

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

    let mut scaled_dragon = Instance::new(Arc::new(load_dragon()));
    scaled_dragon.rotate(Vector3::new(0.0, 1.0, 0.0), PI);
    scaled_dragon.scale_by_scalar(2.5);
    scaled_dragon.translate(Vector3::new(0.0, -scaled_dragon.get_bounds().min.y, 0.0));
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
        PI / 6.0,
        (height as f32) / (width as f32),
        0.15,
        (camera_center - middle_dragon_center).length(),
    );

    let integrator = MonteCarloPathTrace::new(Arc::new(scene), Color::new(0.7, 0.8, 1.0));
    let renderer = Renderer::new(Arc::new(camera), Arc::new(integrator), samples);
    let image = renderer.render(width, height);
    image.write(&ppm_name);
    println!();
}
