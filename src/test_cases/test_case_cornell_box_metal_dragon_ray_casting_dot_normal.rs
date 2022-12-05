use crate::core::pbrt::*;
use crate::cornell_box::{cornell_box_camera, cornell_box_metal_dragon};

#[allow(dead_code)]
pub fn test(width: usize, height: usize) {
    let file_name = get_file_name(file!());
    println!("TESTING: {}", &file_name);

    let integrator = DebuggerRayCastingDotNormal::new(Arc::new(cornell_box_metal_dragon()));
    let renderer = Renderer::new(
        Arc::new(cornell_box_camera(width, height)),
        Arc::new(integrator),
        1,
    );
    let image = renderer.render(width, height);
    image.write(&file_name);
    println!();
}
