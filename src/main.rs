mod fundamental;
mod ray_tracing;
mod test_cases;

use crate::test_cases::primitives_intersection;

fn main() {
    primitives_intersection::test();
}
