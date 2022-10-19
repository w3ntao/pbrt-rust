use crate::cameras::depth_of_field::DepthOfField;
use crate::core::pbrt::*;
use crate::integrators::monte_carlo_path_trace::MonteCarloPathTrace;
use crate::materials::lambertian::*;
use crate::primitives::sphere::Sphere;
use crate::textures::noise_texture::NoiseTexture;
use crate::utility::get_file_name;
use std::sync::Arc;

#[allow(dead_code)]
pub fn test(width: usize, height: usize, samples: u32) {
    let samples = ((samples as f32).sqrt() as u32).pow(2);
    let file_name = get_file_name(file!());
    let ppm_name = format!("{}_{}.ppm", file_name, samples);
    println!("TESTING: {} for {} samples per pixel", &file_name, samples);

    let mut scene = World::default();

    let perlin_texture = Arc::new(NoiseTexture::new(4.0));

    let lambertian_perlin = Arc::new(Lambertian::new(perlin_texture.clone()));

    let mut big_sphere = Sphere::new(Point::new(0.0, -1000.0, 0.0), 1000.0);
    big_sphere.set_material(lambertian_perlin.clone());
    scene.add(Arc::new(big_sphere));

    let sphere_center = Point::new(0.0, 2.0, 0.0);
    let mut small_sphere = Sphere::new(sphere_center, 2.0);
    small_sphere.set_material(lambertian_perlin.clone());
    scene.add(Arc::new(small_sphere));

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
        0.002,
        (camera_center - sphere_center).length(),
    );

    let integrator = MonteCarloPathTrace::new(Arc::new(scene), Color::new(0.7, 0.8, 1.0));
    let renderer = Renderer::new(Arc::new(camera), Arc::new(integrator), samples);
    let image = renderer.render(width, height);
    image.write(&ppm_name);
    println!();
}
