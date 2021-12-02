use std::fs;
use std::fs::OpenOptions;
use std::io::prelude::*;
use crate::perspective_camera::PerspectiveCamera;
use crate::ray_casting_integrator::RayCastingIntegrator;
use crate::renderer::Renderer;

mod vector;
mod triangle;
mod ray;
mod intersection;
mod perspective_camera;
mod ray_casting_integrator;
mod group;
mod renderer;

use crate::vector::*;

fn main() {
    const IMAGE_WIDTH: usize = 256;
    const IMAGE_HEIGHT: usize = 256;

    let camera = PerspectiveCamera::new(
        Vector::new(0.0, 0.0, 10.0),
        Vector::new(0.0, 0.0, -1.0),
        Vector::new(0.0, 1.0, 0.0),
        std::f32::consts::PI / 4.0,
        std::f32::consts::PI / 3.0);

    let integrator = RayCastingIntegrator::new();

    let renderer = Renderer::new(camera, integrator);
    let pixels = renderer.dummy_render(IMAGE_WIDTH, IMAGE_HEIGHT);
    {
        let ppm_head = format!("P3\n{} {}\n255\n", pixels[0].len(), pixels.len());
        let ppm_file_name = "out.ppm";
        fs::write(ppm_file_name, ppm_head)
            .expect(&format!("Failed to write to `{}`", ppm_file_name));

        let mut file = OpenOptions::new()
            .append(true)
            .open(ppm_file_name)
            .unwrap();
        for h in (0usize..IMAGE_HEIGHT).rev() {
            for w in 0usize..IMAGE_WIDTH {
                write!(file, "{} {} {}\n",
                       pixels[h][w].x as i32,
                       pixels[h][w].y as i32,
                       pixels[h][w].z as i32)
                    .expect(&format!("Failed to append to `{}`", ppm_file_name));
            }
        }
    }
}
