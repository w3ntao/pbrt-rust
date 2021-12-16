use std::cmp::{min, max};
use std::time::Instant;
use std::sync::{Arc, Mutex};
use std::thread;
use std::thread::JoinHandle;

use rand::thread_rng;
use rand::seq::SliceRandom;

extern crate num_cpus;

use crate::fundamental::image::Image;
use crate::ray_tracing::camera::Camera;
use crate::ray_tracing::integrator::Integrator;

const MIN_BATCH_SIZE: usize = 128;

#[derive(Clone)]
struct Job {
    pub x: usize,
    pub y: usize,
}

pub struct Renderer {
    camera: Arc<dyn Camera>,
    integrator: Arc<dyn Integrator>,
}

impl Renderer {
    pub fn new(_camera: Arc<dyn Camera>, _integrator: Arc<dyn Integrator>) -> Self {
        return Self {
            camera: _camera,
            integrator: _integrator,
        };
    }

    fn single_thread_render(&self,
                            image: &mut Arc<Mutex<Image>>,
                            job_list: &mut Arc<Mutex<Vec<Vec<Job>>>>) {
        let locked_image = image.lock().unwrap();
        let width = locked_image.width;
        let height = locked_image.height;
        std::mem::drop(locked_image);

        loop {
            let mut locked_job = job_list.lock().unwrap();
            let maybe_job = locked_job.pop();
            std::mem::drop(locked_job);

            match maybe_job {
                Some(job_batch) => {
                    for job in job_batch.iter() {
                        let x = job.x;
                        let y = job.y;
                        let ndc_y = -2.0 * (y as f32) / (height as f32) + 1.0;
                        let ndc_x = 2.0 * (x as f32) / (width as f32) - 1.0;

                        let ray = self.camera.get_primary_ray(
                            ndc_x + 1.0 / (width as f32),
                            ndc_y - 1.0 / (height as f32));
                        let color = self.integrator.get_radiance(&ray);

                        let mut locked_image = image.lock().unwrap();
                        locked_image.fill(color, y, x);
                        std::mem::drop(locked_image);
                    }
                }
                None => break,
            };
        }
    }

    pub fn render(self, width: usize, height: usize) -> Image {
        let start = Instant::now();

        let mut all_jobs: Vec<Job> = vec![];
        all_jobs.reserve(width * height);
        for _x in 0..width {
            for _y in 0..height {
                all_jobs.push(Job { x: _x, y: _y });
            }
        }
        all_jobs.shuffle(&mut thread_rng());
        let all_jobs = all_jobs;

        let batch_size = max(all_jobs.len() / num_cpus::get_physical() / 1000, MIN_BATCH_SIZE);
        // Every core executes roughly 1000 batches of jobs.
        // Also, number of jobs for each batch shouldn't be smaller than MIN_BATCH_SIZE

        let mut job_list: Vec<Vec<Job>> = vec![];
        for idx in (0..all_jobs.len()).step_by(batch_size) {
            let batch = &all_jobs[idx..min(idx + batch_size, all_jobs.len())];
            job_list.push(batch.clone().to_vec());
        }
        let shared_job = Arc::new(Mutex::new(job_list));
        let shared_image = Arc::new(Mutex::new(Image::new(width, height)));

        let mut handles: Vec<JoinHandle<()>> = vec![];
        let arc_self = Arc::new(self);
        for _ in 0..num_cpus::get_physical() {
            let mut image_ptr = Arc::clone(&shared_image);
            let mut job_ptr = Arc::clone(&shared_job);

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
