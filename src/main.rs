use crate::test_cases::{test_case_0_primitives_intersection, test_case_4_material_b};
use crate::test_cases::test_case_1_bvh_building;
use crate::test_cases::test_case_2_multiple_instances;
use crate::test_cases::test_case_3_material_a;

mod fundamental;
mod ray_tracing;
mod test_cases;

fn main() {
    println!();
    //test_case_0_primitives_intersection::test();
    //test_case_1_bvh_building::test();
    //test_case_2_multiple_instances::test();
    //test_case_3_material_a::test();
    test_case_4_material_b::test();
}
