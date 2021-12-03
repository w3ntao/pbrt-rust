mod fundamental;
mod ray_tracing;
mod test_cases;

use crate::test_cases::solids_intersection;

fn main() {
    solids_intersection::test();
}
