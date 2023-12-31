extern crate num_cpus;

use crate::core::pbrt::*;

fn print_time(seconds: u32) {
    let hour = seconds / 3600;
    let minute = (seconds % 3600) / 60;
    let seconds = seconds % 60;

    if hour > 0 {
        print!("{}h {}m", hour, minute);
        return;
    }

    if minute > 0 {
        print!("{}m {}s", minute, seconds);
        return;
    }

    print!("{}s", seconds);
}

fn time_estimator(
    shared_job_list: &mut Arc<Mutex<Vec<usize>>>,
    total_job: usize,
    core: usize,
    file_name: &str,
) {
    let start = Instant::now();
    let one_second = time::Duration::from_secs(1);

    let mut last_length = total_job;
    print!("rendering `{}`:  0.00%  (time left: ?)", file_name);
    let _ = io::stdout().flush();

    let spaces = (0..96).map(|_| " ").collect::<String>();

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
        let seconds_left =
            start.elapsed().as_secs_f32() / (finished_job as f32) * ((length + core) as f32);
        print!("\r{}", spaces);
        print!(
            "\rrendering `{}`: {:5.2}% (time left: ",
            file_name,
            finished_job as f32 / total_job as f32 * 100.0
        );
        print_time(seconds_left as u32);
        print!(")");
        let _ = io::stdout().flush();
    }
    print!("\r{}", spaces);
    print!("\rrendering `{}` took ", file_name,);
    print_time(start.elapsed().as_secs_f32() as u32);
    println!();
}

fn single_thread_render(
    configuration: Arc<Configuration>,
    num_samples: u32,
    image: &mut Arc<Mutex<Image>>,
    shared_job_list: &mut Arc<Mutex<Vec<usize>>>,
) {
    let locked_image = image.lock().unwrap();
    let width = locked_image.width;
    let height = locked_image.height;
    drop(locked_image);

    let mut rendered_pixels: Vec<(usize, usize, Color)> = vec![];

    let mut forked_sampler = configuration.sampler.fork();
    let mutated_sampler = forked_sampler.as_mut();
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

                    mutated_sampler.preprocess(num_samples as usize);
                    for _ in 0..num_samples {
                        let ray = configuration.camera.get_ray(
                            ndc_x,
                            ndc_y,
                            width,
                            height,
                            mutated_sampler,
                        );

                        total += configuration.integrator.get_radiance(
                            ray,
                            configuration.scene.clone(),
                            mutated_sampler,
                        );
                        mutated_sampler.update_round();
                    }

                    let color = total / (num_samples as f32);
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

pub fn render(
    configuration: Arc<Configuration>,
    num_samples: u32,
    width: usize,
    height: usize,
    file_name: &str,
) {
    let mut job_list: Vec<usize> = (0..width).collect();
    job_list.shuffle(&mut thread_rng());

    let shared_job_list = Arc::new(Mutex::new(job_list));
    let shared_image = Arc::new(Mutex::new(Image::new(width, height)));

    let mut handles: Vec<JoinHandle<()>> = vec![];
    let cpu_num = num_cpus::get_physical();
    for _ in 0..cpu_num {
        let mut image_ptr = Arc::clone(&shared_image);
        let mut job_ptr = Arc::clone(&shared_job_list);
        let cloned_configuration = configuration.clone();

        let handle = thread::spawn(move || {
            single_thread_render(
                cloned_configuration,
                num_samples,
                &mut image_ptr,
                &mut job_ptr,
            )
        });
        handles.push(handle);
    }
    let mut job_ptr = Arc::clone(&shared_job_list);
    let file_name_string: String = file_name.into();
    let handle_time_estimator =
        thread::spawn(move || time_estimator(&mut job_ptr, width, cpu_num, &file_name_string));
    handles.push(handle_time_estimator);

    for handle in handles {
        handle.join().unwrap();
    }

    match Arc::try_unwrap(shared_image) {
        Ok(locked_image) => {
            locked_image.into_inner().unwrap().write(file_name);
            println!("image saved to `{}.png`\n", &file_name);
        }
        Err(_) => {
            panic!("Renderer: fail to unwrap rendered image");
        }
    }
}
