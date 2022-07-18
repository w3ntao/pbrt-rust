use std::sync::Arc;

use crate::fundamental::utility::*;
use crate::ray_tracing::cameras::depth_of_field::DepthOfField;
use crate::ray_tracing::integrators::monte_carlo_path_trace::MonteCarloPathTrace;
use crate::ray_tracing::materials::lambertian::*;
use crate::ray_tracing::primitive::Primitive;
use crate::ray_tracing::primitives::sphere::Sphere;
use crate::ray_tracing::renderer::Renderer;
use crate::ray_tracing::textures::noise_texture::NoiseTexture;
use crate::ray_tracing::world::World;

#[allow(dead_code)]
pub fn test(samples: u32) {
    let file_name = get_file_name(file!());
    println!("TESTING: {}", &file_name);
    let ppm_name = format!("{}.ppm", file_name);

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

    let width = 1000;
    let height = 750;

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
