use crate::test_cases::*;

mod fundamental;
mod ray_tracing;
mod test_cases;

fn main() {
    let samples = 5;
    println!();
    test_case_01_primitives_intersection::test();
    test_case_02_bvh_building::test();
    test_case_03_multiple_instances::test();
    test_case_04_material_a::test(samples);
    test_case_05_material_b::test(samples);
    test_case_06_depth_of_field::test(samples);
    test_case_07_rt_weekend_final::test(samples);
    test_case_08_rt_weekend_final_dragon::test(samples);
    test_case_09_checker_texture::test(samples);
    test_case_10_perlin_texture::test(samples);
    test_case_11_lighting::test(samples);
    test_case_12_cornell_box_monte_carlo::test(samples);
    test_case_13_cornell_box_next_event_estimation::test(samples);
    test_case_14_cornell_box_specular_monte_carlo::test(samples);
    test_case_15_cornell_box_specular_next_event_estimation::test(samples);
    test_case_16_cornell_box_metal_dragon_next_event_estimation::test(samples);
    test_case_17_smallpt::test(samples);
}
