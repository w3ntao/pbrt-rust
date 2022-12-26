#![feature(unboxed_closures, fn_traits)]

use crate::test_cases::*;

mod accelerators;
mod cameras;
mod core;
mod integrators;
mod materials;
mod samplers;
mod shapes;
mod test_cases;
mod tools;

fn all_tests(samples: u32, ratio: f32) {
    println!();
    let width = (1920 as f32 * ratio) as usize;
    let height = (1080 as f32 * ratio) as usize;

    //test_case_dragon_bvh::test(width, height);
    //test_case_dragon_transformed::test(width, height);
    test_case_many_dragons::test(width, height);

    test_case_rt_weekend_dragon_pt::test(width, height, samples);
    test_case_rt_weekend_dragon_debug::test(width, height, samples);

    let width = (1080 as f32 * ratio) as usize;
    let height = (1080 as f32 * ratio) as usize;

    test_case_cornell_box_metal_dragon_debug::test(width, height);
    test_case_cornell_box_metal_dragon_next_event_estimation::test(width, height, samples);

    test_case_cornell_box_next_event_estimation::test(width, height, samples);
    test_case_cornell_box_specular_next_event_estimation::test(width, height, samples);

    test_case_cornell_box_specular_path_trace::test(width, height, samples);

    test_case_smallpt::test(
        (2048 as f32 * ratio) as usize,
        (1524 as f32 * ratio) as usize,
        samples,
    );
    // original dimension for smallpt is 1024x762
}

fn main() {
    all_tests(10, 1.0);
}
