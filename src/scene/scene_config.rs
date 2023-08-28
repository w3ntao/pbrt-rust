use crate::pbrt::*;

pub struct SceneConfig {
    integrator: Arc<dyn Integrator>,
    aggregate: Arc<BVHAggregate>,
    sampler: Arc<dyn Sampler>,
    camera: Arc<dyn Camera>,
    film: Arc<Mutex<SimpleRGBFilm>>,
}

fn single_thread_render(
    film: &mut Arc<Mutex<SimpleRGBFilm>>,
    job_list: &mut Arc<Mutex<Vec<i32>>>,
    num_samples: usize,
    integrator: Arc<dyn Integrator>,
    aggregate: Arc<BVHAggregate>,
    sampler: Arc<dyn Sampler>,
    camera: Arc<dyn Camera>,
) {
    let mut forked_sampler = sampler.fork();
    let mutated_sampler = forked_sampler.as_mut();

    let resolution = film.lock().unwrap().resolution;
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
                        integrator.evaluate_pixel_sample(
                            pixel,
                            sample_index,
                            aggregate.clone(),
                            mutated_sampler,
                            camera.clone(),
                            &mut film.clone(),
                        );
                    }
                }
            }
        }
    }
}

impl SceneConfig {
    pub fn new(
        integrator: Arc<dyn Integrator>,
        aggregate: Arc<BVHAggregate>,
        sampler: Arc<dyn Sampler>,
        camera: Arc<dyn Camera>,
        film: Arc<Mutex<SimpleRGBFilm>>,
    ) -> Self {
        return SceneConfig {
            integrator,
            aggregate,
            sampler,
            camera,
            film,
        };
    }

    pub fn render(&mut self) -> usize {
        let resolution = self.film.lock().unwrap().resolution;

        let job_list = Arc::new(Mutex::new((0..resolution.y).collect::<Vec<i32>>()));

        let mut handles: Vec<JoinHandle<()>> = vec![];
        let cpu_num = num_cpus::get();

        let num_samples = 20;
        for _ in 0..cpu_num {
            let mut shared_film = self.film.clone();
            let mut shared_job_list = job_list.clone();

            let integrator = self.integrator.clone();
            let aggregate = self.aggregate.clone();
            let sampler = self.sampler.clone();
            let camera = self.camera.clone();

            let handle = thread::spawn(move || {
                single_thread_render(
                    &mut shared_film,
                    &mut shared_job_list,
                    num_samples,
                    integrator,
                    aggregate,
                    sampler,
                    camera,
                )
            });
            handles.push(handle);
        }

        for handle in handles {
            handle.join().unwrap();
        }

        self.film.lock().unwrap().save_image();

        return cpu_num;
    }
}
