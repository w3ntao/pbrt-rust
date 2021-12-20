mod fundamental;
mod ray_tracing;
mod test_cases;

use crate::test_cases::test_case_0_primitives_intersection;
use crate::test_cases::test_case_1_bvh_building;
use crate::test_cases::test_case_2_multiple_instances;
use crate::test_cases::test_case_3_material;

fn main() {
    println!();
    //primitives_intersection::test();
    //bvh_building::test();
    //multiple_instances::test();
    test_case_3_material::test();
}
