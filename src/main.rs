use crate::test_cases::*;

mod core;
mod fundamental;
mod ray_tracing;
mod test_cases;

fn all_tests(samples: u32) {
    println!();

    let mut width = 500;
    let mut height = 375;

    test_case_01_primitives_intersection::test(width, height);
    test_case_02_bvh_building::test(width, height);
    test_case_03_multiple_instances::test(width, height);
    test_case_04_material_a::test(width, height, samples);
    test_case_05_material_b::test(width, height, samples);
    test_case_06_depth_of_field::test(width, height, samples);
    test_case_07_rt_weekend_final::test(width, height, samples);
    test_case_08_rt_weekend_final_dragon::test(width, height, samples);
    test_case_09_checker_texture::test(width, height, samples);
    test_case_10_perlin_texture::test(width, height, samples);
    test_case_11_lighting::test(width, height, samples);

    width = 500;
    height = 500;

    test_case_12_cornell_box_monte_carlo::test(width, height, samples);
    test_case_13_cornell_box_next_event_estimation::test(width, height, samples);
    test_case_14_cornell_box_specular_monte_carlo::test(width, height, samples);
    test_case_15_cornell_box_specular_next_event_estimation::test(width, height, samples);

    test_case_16_cornell_box_metal_dragon_ray_casting_normal::test(width, height, 1);
    test_case_17_cornell_box_metal_dragon_ray_casting_dot_normal::test(width, height, 1);
    test_case_18_cornell_box_metal_dragon_next_event_estimation::test(width, height, samples);

    test_case_19_smallpt::test(1024, 762, samples);
}

fn main() {
    all_tests(5);
}
