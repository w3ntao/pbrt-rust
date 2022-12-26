use crate::core::pbrt::*;
use crate::cornell_box::{cornell_box_camera, cornell_box_specular};

#[allow(dead_code)]
pub fn test(width: usize, height: usize, samples: u32) {
    let samples = ((samples as f32).sqrt() as u32).pow(2);
    let file_name = get_file_name(file!());
    println!("TESTING: {} for {} samples per pixel", &file_name, samples);

    let integrator = PathTrace::default();
    let renderer = Renderer::new(
        Arc::new(cornell_box_specular()),
        Arc::new(cornell_box_camera(width, height)),
        Arc::new(integrator),
        Arc::new(StratifiedSampler::default()),
        samples,
    );
    let image = renderer.render(width, height);
    image.write(&format!("{}_{}", file_name, samples));
    println!();
}
