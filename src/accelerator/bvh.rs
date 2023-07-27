use crate::pbrt::*;

pub struct BVHAggregate {
    primitives: Vec<Arc<dyn Primitive>>,
}

impl Primitive for BVHAggregate {
    fn intersect(&self, ray: &Ray, t_max: Float) -> Option<ShapeIntersection> {
        // TODO: should rewrite this
        for primitive in &self.primitives {
            match primitive.intersect(&ray, Float::INFINITY) {
                None => {
                    continue;
                }

                Some(shape_intersection) => {
                    return Some(shape_intersection);
                }
            }
        }

        return None;
    }

    fn get_bounds(&self) -> Bounds3f {
        panic!("not implemented");
    }
}

impl BVHAggregate {
    pub fn build_bvh(primitives: Vec<Arc<dyn Primitive>>) -> BVHAggregate {
        return BVHAggregate { primitives };
    }
}
