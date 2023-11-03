use crate::pbrt::*;
use std::thread;
use std::thread::JoinHandle;

pub struct Renderer {
    integrator: Arc<dyn Integrator>,
    sampler: Arc<dyn Sampler>,
    camera: Arc<dyn Camera>,
    film: Arc<Mutex<dyn Film>>,
}

fn single_thread_render(
    film: &mut Arc<Mutex<dyn Film>>,
    job_list: &mut Arc<Mutex<Vec<i32>>>,
    num_samples: usize,
    integrator: Arc<dyn Integrator>,
    sampler: Arc<dyn Sampler>,
    camera: Arc<dyn Camera>,
) {
    let mut forked_sampler = sampler.fork();
    let mutated_sampler = forked_sampler.as_mut();

    let filter = film.lock().unwrap().get_filter().clone();

    let resolution = film.lock().unwrap().get_resolution();
    loop {
        let mut locked_job_list = job_list.lock().unwrap();
        let maybe_job = locked_job_list.pop();
        drop(locked_job_list);

        match maybe_job {
            None => {
                break;
            }
            Some(y) => {
                for x in 0..resolution.x {
                    let pixel = Point2i::new(x, y);

                    for sample_index in 0..num_samples {
                        mutated_sampler.start_pixel_sample(pixel, sample_index);
                        integrator.evaluate_pixel_sample(
                            pixel,
                            mutated_sampler,
                            camera.clone(),
                            filter.clone(),
                            &mut film.clone(),
                        );
                    }
                }
            }
        }
    }
}

impl Renderer {
    pub fn new(
        integrator: Arc<dyn Integrator>,
        sampler: Arc<dyn Sampler>,
        camera: Arc<dyn Camera>,
        film: Arc<Mutex<dyn Film>>,
    ) -> Self {
        return Renderer {
            integrator,
            sampler,
            camera,
            film,
        };
    }

    pub fn render(&mut self, num_samples: usize, num_cores: usize) {
        let resolution = self.film.lock().unwrap().get_resolution();

        let job_list = Arc::new(Mutex::new((0..resolution.y).collect::<Vec<i32>>()));

        let mut handles: Vec<JoinHandle<()>> = vec![];

        for _ in 0..num_cores {
            let mut shared_film = self.film.clone();
            let mut shared_job_list = job_list.clone();

            let integrator = self.integrator.clone();
            let sampler = self.sampler.clone();
            let camera = self.camera.clone();

            let handle = thread::spawn(move || {
                single_thread_render(
                    &mut shared_film,
                    &mut shared_job_list,
                    num_samples,
                    integrator,
                    sampler,
                    camera,
                )
            });
            handles.push(handle);
        }

        for handle in handles {
            handle.join().unwrap();
        }

        let filename = self.film.lock().unwrap().get_filename();
        let resolution = self.film.lock().unwrap().get_resolution();

        self.film
            .lock()
            .unwrap()
            .export_image(&filename, resolution);
    }
}
