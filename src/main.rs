use std::fs;
use std::fs::OpenOptions;
use std::io::prelude::*;

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

    let mut pixels = [[Color::zero(); IMAGE_WIDTH]; IMAGE_HEIGHT];
    let factor = 256 as f32 - 0.001;
    for h in (0usize..IMAGE_HEIGHT).rev() {
        for w in 0usize..IMAGE_WIDTH {
            let red = w as f32 / (IMAGE_WIDTH - 1) as f32;
            let green = h as f32 / (IMAGE_HEIGHT - 1) as f32;
            let blue = 0.25;

            pixels[h][w] = Vector::new(red * factor,
                                       green * factor,
                                       blue * factor);
        }
    }
    let pixels = pixels;
    //discard mut
    {
        let ppm_head = format!("P3\n{} {}\n255\n", IMAGE_WIDTH, IMAGE_HEIGHT);
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
