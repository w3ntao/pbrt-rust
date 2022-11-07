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

fn all_tests(samples: u32) {
    println!();

    let width = 1000;
    let height = 750;

    test_case_rt_weekend_final_dragon::test(width, height, samples);

    let width = 1000;
    let height = 1000;

    test_case_cornell_box_metal_dragon_next_event_estimation::test(width, height, samples);
    test_case_cornell_box_next_event_estimation::test(width, height, samples);
    test_case_cornell_box_specular_path_trace::test(width, height, samples);
    test_case_cornell_box_specular_next_event_estimation::test(width, height, samples);

    test_case_smallpt::test(1024, 762, samples);
}

fn main() {
    all_tests(10);
}
