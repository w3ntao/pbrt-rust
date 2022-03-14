use crate::test_cases::*;

mod fundamental;
mod ray_tracing;
mod test_cases;

fn main() {
    println!();
    //test_case_0_primitives_intersection::test();
    //test_case_1_bvh_building::test();
    //test_case_2_multiple_instances::test();
    //test_case_3_material_a::test(2000);
    //test_case_4_material_b::test(2000);
    test_case_5_depth_of_field::test(2000);
}
