use crate::core::pbrt::*;
use crate::test_case_rt_weekend_dragon_pt::{many_random_spheres_with_dragons, rt_weekend_camera};

#[allow(dead_code)]
pub fn test(width: usize, height: usize, samples: u32) {
    let samples = ((samples as f32).sqrt() as u32).pow(2);
    let file_name = get_file_name(file!());
    println!("TESTING: {} for {} samples per pixel", &file_name, samples);

    let integrator = DebuggerScatterRay::default();
    let renderer = Renderer::new(
        Arc::new(many_random_spheres_with_dragons()),
        rt_weekend_camera(width, height),
        Arc::new(integrator),
        samples,
    );
    let image = renderer.render(width, height);
    image.write(&format!("{}_{}", file_name, samples));
    println!();
}
