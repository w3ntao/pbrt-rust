use std::sync::Arc;

use crate::fundamental::utility::*;
use crate::ray_tracing::cameras::depth_of_field::DepthOfField;
use crate::ray_tracing::group::Group;
use crate::ray_tracing::groups::bvh::BVH;
use crate::ray_tracing::instance::Instance;
use crate::ray_tracing::integrators::monte_carlo_path_trace::MonteCarloPathTrace;
use crate::ray_tracing::integrators::ray_casting_dot_normal;
use crate::ray_tracing::integrators::ray_casting_dot_normal::RayCastingDotNormal;
use crate::ray_tracing::integrators::ray_casting_normal::RayCastingNormal;
use crate::ray_tracing::materials::lambertian::*;
use crate::ray_tracing::primitive::Primitive;
use crate::ray_tracing::primitives::axis_aligned_box::AxisAlignedBox;
use crate::ray_tracing::primitives::quad::Quad;
use crate::ray_tracing::primitives::sphere::Sphere;
use crate::ray_tracing::renderer::Renderer;
use crate::ray_tracing::textures::diffuse_light::DiffuseLight;
use crate::ray_tracing::textures::noise_texture::NoiseTexture;
use crate::ray_tracing::textures::solid_color::SolidColor;
use crate::ray_tracing::world::World;

pub fn test(samples: u32) {
    let file_name = get_file_name(file!());
    println!("TESTING: {}", &file_name);
    let ppm_name = format!("{}.ppm", file_name);

    const WIDTH: usize = 600;
    const HEIGHT: usize = 600;

    let mut scene = BVH::default();

    /*
    let diffuse_light = DiffuseLight::new(Arc::new(SolidColor::new(Color::new(4.0, 4.0, 4.0))));
    let mut quad_light = Quad::new(Point::new(3.0, 1.0, -2.0), Vector3::new(2.0, 0.0, 0.0), Vector3::new(0.0, 2.0, 0.0));
    quad_light.set_material(Arc::new(diffuse_light));
    scene.add(Arc::new(quad_light));
    */

    let solid_color_red = Arc::new(SolidColor::new(Color::new(0.65, 0.05, 0.05)));
    let solid_color_green = Arc::new(SolidColor::new(Color::new(0.12, 0.45, 0.15)));
    let solid_color_white = Arc::new(SolidColor::new(Color::new(0.73, 0.73, 0.73)));

    let lambertian_red = Arc::new(Lambertian::new(solid_color_red.clone()));
    let lambertian_green = Arc::new(Lambertian::new(solid_color_green.clone()));
    let lambertian_white = Arc::new(Lambertian::new(solid_color_white.clone()));

    let length = 555.0;
    let mut wall_left = Quad::new(Point::new(length, 0.0, 0.0), Vector3::new(0.0, length, 0.0), Vector3::new(0.0, 0.0, length));
    wall_left.set_material(lambertian_green.clone());
    let wall_left = Arc::new(wall_left);
    scene.add(wall_left.clone());

    let mut wall_right = Quad::new(Point::new(0.0, 0.0, 0.0), Vector3::new(0.0, length, 0.0), Vector3::new(0.0, 0.0, length));
    wall_right.set_material(lambertian_red.clone());
    let wall_right = Arc::new(wall_right);
    scene.add(wall_right.clone());

    let mut wall_back = Quad::new(Point::new(0.0, 0.0, length), Vector3::new(0.0, length, 0.0), Vector3::new(length, 0.0, 0.0));
    wall_back.set_material(lambertian_white.clone());
    let wall_back = Arc::new(wall_back);
    scene.add(wall_back.clone());

    let mut wall_bottom = Quad::new(Point::new(0.0, 0.0, 0.0), Vector3::new(length, 0.0, 0.0), Vector3::new(0.0, 0.0, length));
    wall_bottom.set_material(lambertian_white.clone());
    let wall_bottom = Arc::new(wall_bottom);
    scene.add(wall_bottom.clone());

    let mut wall_up = Quad::new(Point::new(0.0, length, 0.0), Vector3::new(length, 0.0, 0.0), Vector3::new(0.0, 0.0, length));
    wall_up.set_material(lambertian_white.clone());
    let wall_up = Arc::new(wall_up);
    scene.add(wall_up.clone());


    let mut box_a = AxisAlignedBox::new(
        Point::new(0.0, 0.0, 0.0),
        Point::new(165.0, 330.0, 165.0));
    let mut box_a = Instance::new(Arc::new(box_a));
    box_a.rotate(Vector3::new(0.0, 1.0, 0.0), PI / 12.0);
    box_a.translate(Vector3::new(265.0, 0.0, 295.0));
    box_a.set_material(lambertian_white.clone());
    let box_a = Arc::new(box_a);
    scene.add(box_a.clone());

    let mut box_b = AxisAlignedBox::new(
        Point::new(0.0, 0.0, 0.0),
        Point::new(165.0, 165.0, 165.0));
    let mut box_b = Instance::new(Arc::new(box_b));
    box_b.rotate(Vector3::new(0.0, 1.0, 0.0), -PI / 10.0);
    box_b.translate(Vector3::new(130.0, 0.0, 65.0));
    box_b.set_material(lambertian_white.clone());
    let box_b = Arc::new(box_b);
    scene.add(box_b.clone());

    let diffuse_light = DiffuseLight::new(Arc::new(SolidColor::new(Color::new(15.0, 15.0, 15.0))));
    let mut quad_light = Quad::new(Point::new(213.0, length - 1.0, 227.0), Vector3::new(130.0, 0.0, 0.0), Vector3::new(0.0, 0.0, 105.0));
    quad_light.set_material(Arc::new(diffuse_light));
    let quad_light = Arc::new(quad_light);
    scene.add(quad_light);
    scene.build_index();

    let camera_center = Point::new(278.0, 278.0, -800.0);
    let look_at = Point::new(278.0, 278.0, 0.0);
    let direction = look_at - camera_center;

    let camera = DepthOfField::new(
        camera_center,
        direction,
        Vector3::new(0.0, 1.0, 0.0),
        PI / 4.0,
        PI / 4.0,
        0.000001, (look_at - camera_center).length());

    let world = World::new(Arc::new(scene));
    let integrator = MonteCarloPathTrace::new(Arc::new(world), Color::black());
    let renderer = Renderer::new(Arc::new(camera), Arc::new(integrator), samples);
    let image = renderer.render(WIDTH, HEIGHT);
    image.write(&ppm_name);
    println!();
}
