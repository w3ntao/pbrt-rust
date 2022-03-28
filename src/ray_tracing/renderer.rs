extern crate num_cpus;

use std::cmp::{max, min};
use std::sync::{Arc, Mutex};
use std::thread;
use std::thread::JoinHandle;
use std::time::Instant;

use rand::Rng;
use rand::seq::SliceRandom;
use rand::thread_rng;

use crate::fundamental::color::Color;
use crate::fundamental::image::Image;
use crate::ray_tracing::camera::Camera;
use crate::ray_tracing::integrator::Integrator;

pub struct Renderer {
    camera: Arc<dyn Camera>,
    integrator: Arc<dyn Integrator>,
    samples: i32,
}

impl Renderer {
    pub fn new(_camera: Arc<dyn Camera>, _integrator: Arc<dyn Integrator>, _samples: i32) -> Self {
        return Self {
            camera: _camera,
            integrator: _integrator,
            samples: _samples,
        };
    }

    fn single_thread_render(&self,
                            image: &mut Arc<Mutex<Image>>,
                            shared_job_list: &mut Arc<Mutex<Vec<usize>>>) {
        let locked_image = image.lock().unwrap();
        let width = locked_image.width;
        let height = locked_image.height;
        std::mem::drop(locked_image);
        let mut rng = rand::thread_rng();

        let mut rendered_pixels: Vec<(usize, usize, Color)> = vec![];

        loop {
            let mut locked_job = shared_job_list.lock().unwrap();
            let maybe_x = locked_job.pop();
            std::mem::drop(locked_job);

            match maybe_x {
                Some(x) => {
                    for y in 0..height {
                        let ndc_y = -2.0 * (y as f32) / (height as f32) + 1.0;
                        let ndc_x = 2.0 * (x as f32) / (width as f32) - 1.0;

                        if self.samples == 1 {
                            let ray = self.camera.get_primary_ray(
                                ndc_x + 1.0 / (width as f32),
                                ndc_y - 1.0 / (height as f32));
                            let color = self.integrator.get_radiance(&ray);
                            rendered_pixels.push((y, x, color));
                            continue;
                        }

                        let mut total = Color::black();
                        for _ in 0..self.samples {
                            let random_x: f32 = rng.gen();
                            let random_y: f32 = rng.gen();

                            let u = ndc_x + 2.0 * random_x / (width as f32);
                            let v = ndc_y - 2.0 * random_y / (height as f32);
                            // u, v are both in [-1, 1]

                            let ray = self.camera.get_primary_ray(u, v);
                            total = total + self.integrator.get_radiance(&ray);
                        }
                        let color = total / (self.samples as f32);
                        rendered_pixels.push((y, x, color));
                    }
                }
                None => break,
            };
        }

        let mut locked_image = image.lock().unwrap();
        for (y, x, color) in rendered_pixels {
            locked_image.fill(y, x, color);
        }
        std::mem::drop(locked_image);
    }

    pub fn render(self, width: usize, height: usize) -> Image {
        let start = Instant::now();

        let job_list: Vec<usize> = (0..width).collect();
        let shared_job_list = Arc::new(Mutex::new(job_list));
        let shared_image = Arc::new(Mutex::new(Image::new(width, height)));

        let mut handles: Vec<JoinHandle<()>> = vec![];
        let arc_self = Arc::new(self);
        for _ in 0..num_cpus::get_physical() {
            let mut image_ptr = Arc::clone(&shared_image);
            let mut job_ptr = Arc::clone(&shared_job_list);

            let forked_self = arc_self.clone();
            let handle =
                thread::spawn(move ||
                    forked_self.single_thread_render(&mut image_ptr, &mut job_ptr)
                );
            handles.push(handle);
        }
        for handle in handles {
            handle.join().unwrap();
        }

        println!("Rendering took {:.2}[s]", start.elapsed().as_secs_f32());

        match Arc::try_unwrap(shared_image) {
            Ok(locked_image) => {
                return locked_image.into_inner().unwrap();
            }
            Err(_) => {
                panic!("Renderer: fail to return rendered image");
            }
        }
    }
}
