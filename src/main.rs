use crate::test_cases::*;

mod fundamental;
mod ray_tracing;
mod test_cases;

fn main() {
    //let samples = 5;
    println!();
    //test_case_0_primitives_intersection::test();
    //test_case_1_bvh_building::test();
    //test_case_2_multiple_instances::test();
    //test_case_3_material_a::test(samples);
    //test_case_4_material_b::test(samples);
    //test_case_5_depth_of_field::test(samples);
    //test_case_6_rt_weekend_final::test(samples);
    //test_case_7_rt_weekend_final_dragon::test(samples);
    //test_case_8_checker_texture::test(samples);
    //test_case_9_perlin_texture::test(samples);
    //test_case_10_lighting::test(samples);
    //test_case_11_cornell_box::test(samples);
    //test_case_12_cornell_box_sample_light::test(samples);
    //test_case_13_cornell_box_next_event_estimation::test(samples);


    for samples in [50] {
        test_case_12_cornell_box_next_event_estimation::test(samples);
        test_case_14_cornell_box_specular_next_event_estimation::test(samples);
    }

    for samples in [100, 200] {
        test_case_13_cornell_box_specular_monte_carlo::test(samples);
    }
}
