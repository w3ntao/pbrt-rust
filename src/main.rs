#![feature(unboxed_closures, fn_traits)]

use crate::test_cases::*;
mod accelerators;
mod cameras;
mod core;
mod integrators;
mod materials;
mod primitives;
mod test_cases;
mod textures;
mod tools;

fn all_tests(samples: u32) {
    println!();

    let width = 500;
    let height = 375;

    test_case_bvh_building::test(width, height);
    test_case_multiple_instances::test(width, height);
    test_case_material_a::test(width, height, samples);
    test_case_material_b::test(width, height, samples);
    test_case_depth_of_field::test(width, height, samples);
    test_case_rt_weekend_final::test(width, height, samples);
    test_case_rt_weekend_final_dragon::test(width, height, samples);
    test_case_checker_texture::test(width, height, samples);
    test_case_perlin_texture::test(width, height, samples);
    test_case_lighting::test(width, height, samples);

    let width = 500;
    let height = 500;

    test_case_cornell_box_monte_carlo::test(width, height, samples);
    test_case_cornell_box_next_event_estimation::test(width, height, samples);
    test_case_cornell_box_specular_monte_carlo::test(width, height, samples);
    test_case_cornell_box_specular_next_event_estimation::test(width, height, samples);

    test_case_cornell_box_metal_dragon_ray_casting_normal::test(width, height, 1);
    test_case_cornell_box_metal_dragon_ray_casting_dot_normal::test(width, height, 1);
    test_case_cornell_box_metal_dragon_next_event_estimation::test(width, height, samples);

    test_case_smallpt::test(1024, 762, samples);
}

fn main() {
    all_tests(5);
}
