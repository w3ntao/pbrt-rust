use crate::pbrt::*;

pub struct BVHAggregate {
    primitives: Vec<Arc<dyn Primitive>>,
    root: Arc<dyn Primitive>,
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

    fn bounds(&self) -> Bounds3f {
        panic!("not implemented");
    }
}

#[derive(Copy, Clone)]
struct BVHPrimitive {
    primitive_idx: usize,
    centroid: Point3f,
    bounds: Bounds3f,
}

impl BVHPrimitive {
    pub fn new(idx: usize, bounds: Bounds3f) -> Self {
        return BVHPrimitive {
            primitive_idx: idx,
            bounds,
            centroid: 0.5 * (bounds.p_min + bounds.p_max),
        };
    }
}

struct BVHBuildNode {
    bounds: Bounds3f,
    children: [Option<Arc<BVHBuildNode>>; 2],
    split_axis: usize,
    first_primitive_offset: usize,
    primitive_num: usize,
}

impl BVHAggregate {
    pub fn new(primitives: Vec<Arc<dyn Primitive>>) -> BVHAggregate {
        let mut bvh_primitives = vec![];
        for idx in 0..primitives.len() {
            bvh_primitives.push(BVHPrimitive::new(idx, primitives[idx].bounds()));
        }

        let mut ordered_primitives = vec![];
        let root =
            BVHAggregate::build_recursive(&primitives, &bvh_primitives, &mut ordered_primitives);

        println!("\n\nBVH built");
        println!("bvh_primitives:     {}", bvh_primitives.len());
        println!("ordered_primitives: {}", ordered_primitives.len());

        exit(0);
    }

    fn build_leaf(
        first_primitive_offset: usize,
        primitive_num: usize,
        bounds: Bounds3f,
    ) -> BVHBuildNode {
        return BVHBuildNode {
            bounds,
            children: [None, None],
            split_axis: usize::MAX,
            first_primitive_offset,
            primitive_num,
        };
    }

    fn build_interior(
        split_axis: usize,
        left: Arc<BVHBuildNode>,
        right: Arc<BVHBuildNode>,
    ) -> BVHBuildNode {
        return BVHBuildNode {
            bounds: left.bounds + right.bounds,
            children: [Some(left), Some(right)],
            split_axis,
            first_primitive_offset: usize::MAX,
            primitive_num: 0,
        };
    }

    fn build_recursive(
        primitives: &Vec<Arc<dyn Primitive>>,
        bvh_primitives: &Vec<BVHPrimitive>,
        ordered_primitives: &mut Vec<Arc<dyn Primitive>>,
    ) -> Arc<BVHBuildNode> {
        let full_bounds = bvh_primitives
            .iter()
            .map(|primitive| primitive.bounds)
            .sum::<Bounds3f>();

        let mut closure_build_leaf = || {
            let first_primitive_offset = ordered_primitives.len();
            for idx in 0..bvh_primitives.len() {
                let primitive_idx = bvh_primitives[idx].primitive_idx;
                ordered_primitives.push(primitives[primitive_idx].clone());
            }

            return Arc::new(BVHAggregate::build_leaf(
                first_primitive_offset,
                bvh_primitives.len(),
                full_bounds,
            ));
        };

        if full_bounds.surface_area() == 0.0 || bvh_primitives.len() == 1 {
            return closure_build_leaf();
        }

        let mut centroid_bounds = Bounds3f::empty();
        for primitive in bvh_primitives {
            centroid_bounds = centroid_bounds.union(primitive.centroid);
        }

        let split_axis = centroid_bounds.max_dimension();
        if centroid_bounds.p_min[split_axis] == centroid_bounds.p_max[split_axis] {
            return closure_build_leaf();
        }

        // TODO: to implement the simplest BVH
        let mid_val = (centroid_bounds.p_min[split_axis] + centroid_bounds.p_max[split_axis]) / 2.0;

        let mut left_primitives = vec![];
        let mut right_primitives = vec![];
        for primitive in bvh_primitives {
            if primitive.centroid[split_axis] <= mid_val {
                left_primitives.push(primitive.clone());
            } else {
                right_primitives.push(primitive.clone());
            }
        }

        if left_primitives.len() == 0 || right_primitives.len() == 0 {
            // numerical error: when the difference is too small that you couldn't split primitives with mid_val
            return closure_build_leaf();
        }

        // TODO: to implement the simplest BVH

        let left_child = Self::build_recursive(primitives, &left_primitives, ordered_primitives);
        left_primitives.clear();

        let right_child = Self::build_recursive(primitives, &right_primitives, ordered_primitives);
        right_primitives.clear();

        return Arc::new(Self::build_interior(split_axis, left_child, right_child));
    }
}
