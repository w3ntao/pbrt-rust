#![feature(unboxed_closures, fn_traits)]

use crate::test_cases::*;

mod accelerators;
mod cameras;
mod core;
mod integrators;
mod materials;
mod shapes;
mod test_cases;
mod textures;
mod tools;

fn all_tests(samples: u32, ratio: f32) {
    println!();
    let width = (1960 as f32 * ratio) as usize;
    let height = (1080 as f32 * ratio) as usize;

    //test_case_dragon_bvh::test(width, height);
    //test_case_dragon_transformed::test(width, height);
    test_case_many_dragons::test(width, height);

    test_case_rt_weekend_final_dragon::test(width, height, samples);

    let width = (1080 as f32 * ratio) as usize;
    let height = (1080 as f32 * ratio) as usize;

    //test_case_cornell_box_metal_dragon_ray_casting_dot_normal::test(width, height);

    test_case_cornell_box_metal_dragon_next_event_estimation::test(width, height, samples);

    test_case_cornell_box_next_event_estimation::test(width, height, samples);
    test_case_cornell_box_specular_next_event_estimation::test(width, height, samples);

    test_case_cornell_box_specular_path_trace::test(width, height, samples);

    test_case_smallpt::test(
        (1024 as f32 * ratio) as usize,
        (762 as f32 * ratio) as usize,
        samples,
    );
}

fn main() {
    all_tests(10, 1.0);
}
