extern crate num_cpus;

use crate::core::pbrt::*;

pub struct Renderer {
    camera: Arc<dyn Camera>,
    integrator: Arc<dyn Integrator>,
    samples: u32,
}

fn print_time(seconds: i32) {
    if seconds > 3600 {
        let hour = seconds / 3600;
        print!("{}h ", hour);
        let minute = (seconds - hour * 3600) / 60;
        if minute > 0 {
            print!("{}m ", minute);
        }
        return;
    }
    let minute = (seconds) / 60;
    if minute > 0 {
        print!("{}m ", minute);
    }
    let seconds = seconds - minute * 60;
    print!("{}s", seconds);
}

impl Renderer {
    pub fn new(_camera: Arc<dyn Camera>, _integrator: Arc<dyn Integrator>, _samples: u32) -> Self {
        return Self {
            camera: _camera,
            integrator: _integrator,
            samples: _samples,
        };
    }

    fn time_estimator(
        &self,
        shared_job_list: &mut Arc<Mutex<Vec<usize>>>,
        total_job: usize,
        core: usize,
    ) {
        let start = Instant::now();
        let one_second = time::Duration::from_secs(1);

        let mut last_length = total_job;
        print!("time left: estimating...");
        let _ = io::stdout().flush();
        loop {
            thread::sleep(one_second);
            let locked_job = shared_job_list.lock().unwrap();
            let length = locked_job.len();
            drop(locked_job);

            if length == 0 {
                break;
            }
            let finished_job = total_job - core - length;
            if length == last_length || finished_job <= 0 {
                continue;
            }
            last_length = length;
            let unit_job_time = start.elapsed().as_secs_f32() / (finished_job as f32);
            print!("\r                          ");
            print!("\rtime left: ");
            print_time((unit_job_time * ((length + core) as f32)) as i32);
            let _ = io::stdout().flush();
        }

        print!("\nrendering took ");
        print_time(start.elapsed().as_secs_f32() as i32);
        println!();
    }

    fn single_thread_render(
        &self,
        image: &mut Arc<Mutex<Image>>,
        shared_job_list: &mut Arc<Mutex<Vec<usize>>>,
    ) {
        let locked_image = image.lock().unwrap();
        let width = locked_image.width;
        let height = locked_image.height;
        drop(locked_image);

        let mut rendered_pixels: Vec<(usize, usize, Color)> = vec![];
        loop {
            let mut locked_job = shared_job_list.lock().unwrap();
            let maybe_x = locked_job.pop();
            drop(locked_job);

            match maybe_x {
                Some(x) => {
                    for y in 0..height {
                        let ndc_y = -2.0 * (y as f32) / (height as f32) + 1.0;
                        let ndc_x = 2.0 * (x as f32) / (width as f32) - 1.0;
                        let mut total = Color::black();

                        for ray in self.camera.get_stratified_rays(
                            self.samples,
                            ndc_x,
                            ndc_x + 2.0 / (width as f32),
                            ndc_y - 2.0 / (height as f32),
                            ndc_y,
                        ) {
                            total += self.integrator.get_radiance(ray);
                        }

                        let color = total / (self.samples as f32);
                        if !color.is_finite() {
                            panic!(
                                "\n\ninfinite color rendered: {}\nat position (x={}, y={})\n\n",
                                color, x, y
                            );
                        }
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
        drop(locked_image);
    }

    pub fn render(self, width: usize, height: usize) -> Image {
        let mut job_list: Vec<usize> = (0..width).collect();
        job_list.shuffle(&mut thread_rng());

        let shared_job_list = Arc::new(Mutex::new(job_list));
        let shared_image = Arc::new(Mutex::new(Image::new(width, height)));

        let mut handles: Vec<JoinHandle<()>> = vec![];
        let arc_self = Arc::new(self);
        let cpu_num = num_cpus::get_physical();
        for _ in 0..cpu_num {
            let mut image_ptr = Arc::clone(&shared_image);
            let mut job_ptr = Arc::clone(&shared_job_list);

            let forked_self = arc_self.clone();
            let handle = thread::spawn(move || {
                forked_self.single_thread_render(&mut image_ptr, &mut job_ptr)
            });
            handles.push(handle);
        }
        let forked_self = arc_self.clone();
        let mut job_ptr = Arc::clone(&shared_job_list);
        let handle_time_estimator =
            thread::spawn(move || forked_self.time_estimator(&mut job_ptr, width, cpu_num));
        handles.push(handle_time_estimator);

        for handle in handles {
            handle.join().unwrap();
        }

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
