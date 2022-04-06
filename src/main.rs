use crate::test_cases::*;

mod fundamental;
mod ray_tracing;
mod test_cases;

fn main() {
    let samples_per_pixel = 5;

    println!();
    test_case_0_primitives_intersection::test();
    test_case_1_bvh_building::test();
    test_case_2_multiple_instances::test();
    test_case_3_material_a::test(samples_per_pixel);
    test_case_4_material_b::test(samples_per_pixel);
    test_case_5_depth_of_field::test(samples_per_pixel);
    test_case_6_rt_weekend_final::test(samples_per_pixel);
    test_case_7_rt_weekend_final_dragon::test(samples_per_pixel);
    test_case_8_texture::test(samples_per_pixel);
}
