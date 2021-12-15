mod fundamental;
mod ray_tracing;
mod test_cases;

use crate::test_cases::primitives_intersection;
use crate::test_cases::bvh_building;
use crate::test_cases::multiple_instances;

fn main() {
    println!();
    primitives_intersection::test();
    bvh_building::test();
    multiple_instances::test();
}
