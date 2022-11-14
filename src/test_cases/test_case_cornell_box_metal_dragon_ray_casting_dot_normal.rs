use crate::core::pbrt::*;
use crate::cornell_box::{cornell_box_camera, cornell_box_metal_dragon};

#[allow(dead_code)]
pub fn test(width: usize, height: usize) {
    let file_name = get_file_name(file!());
    let ppm_name = format!("{}.ppm", file_name);
    println!("TESTING: {}", &file_name);

    let integrator = RayCastingDotNormal::new(Arc::new(cornell_box_metal_dragon()));
    let renderer = Renderer::new(
        Arc::new(cornell_box_camera(width, height)),
        Arc::new(integrator),
        1,
    );
    let image = renderer.render(width, height);
    image.write(&ppm_name);
    println!();
}
