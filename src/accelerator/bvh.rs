use crate::pbrt::*;

pub struct BVHAggregate {
    ordered_primitives: Vec<Arc<dyn Primitive>>,
    linear_bvh_nodes: Vec<LinearBVHNode>,
}

impl Primitive for BVHAggregate {
    fn intersect(&self, ray: &Ray, t_max: Float) -> Option<ShapeIntersection> {
        let invDir = Vector3f::new(1.0 / ray.d.x, 1.0 / ray.d.y, 1.0 / ray.d.z);
        let dirIsNeg = [
            (invDir.x < 0.0) as usize,
            (invDir.y < 0.0) as usize,
            (invDir.z < 0.0) as usize,
        ];

        let mut nodes_to_visit = vec![0];
        let mut best_t = t_max;
        let mut best_intersection: Option<ShapeIntersection> = None;

        loop {
            let current_node_idx = match nodes_to_visit.pop() {
                None => {
                    break;
                }
                Some(idx) => idx,
            };

            let node = &self.linear_bvh_nodes[current_node_idx];
            if !node.bounds.fast_intersect(ray, best_t, invDir, dirIsNeg) {
                continue;
            }

            if node.primitive_num > 0 {
                for idx in node.offset..(node.offset + node.primitive_num as u32) {
                    let primitive = self.ordered_primitives[idx as usize].clone();
                    match primitive.intersect(ray, best_t) {
                        None => {
                            continue;
                        }
                        Some(si) => {
                            best_t = si.t_hit;
                            best_intersection = Some(si);
                        }
                    }
                }
                continue;
            }

            // interior node
            // Put far BVH node on _nodesToVisit_ stack, advance to near node
            if dirIsNeg[node.axis as usize] > 0 {
                nodes_to_visit.push(current_node_idx + 1);
                nodes_to_visit.push(node.offset as usize);
            } else {
                nodes_to_visit.push(node.offset as usize);
                nodes_to_visit.push(current_node_idx + 1);
            }
        }

        return best_intersection;
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

#[repr(align(32))]
struct LinearBVHNode {
    bounds: Bounds3f,
    offset: u32,
    // if it's a leaf node:     primitives offset
    // if it's a interior node: second child offset
    primitive_num: u16,
    // for leaf node only
    axis: u8,
    // for interior node only: 0, 1, 2 for xyz
}

fn flatten_bvh(node: Arc<BVHBuildNode>, linear_bvh_nodes: &mut Vec<LinearBVHNode>) -> usize {
    let node_offset = linear_bvh_nodes.len();
    linear_bvh_nodes.push(LinearBVHNode {
        bounds: node.bounds,
        offset: 0,
        primitive_num: 0,
        axis: u8::MAX,
    });

    if node.primitive_num > 0 {
        // leaf
        if node.primitive_num > u16::MAX as usize {
            panic!("error: primitive number exceeds limit");
        }
        linear_bvh_nodes[node_offset].offset = node.first_primitive_offset as u32;
        linear_bvh_nodes[node_offset].primitive_num = node.primitive_num as u16;
        return node_offset;
    }

    // Create interior flattened BVH node
    linear_bvh_nodes[node_offset].axis = node.split_axis as u8;
    linear_bvh_nodes[node_offset].primitive_num = 0;
    flatten_bvh(node.children[0].clone().unwrap(), linear_bvh_nodes);
    linear_bvh_nodes[node_offset].offset =
        flatten_bvh(node.children[1].clone().unwrap(), linear_bvh_nodes) as u32;
    return node_offset;
}

impl BVHAggregate {
    pub fn new(primitives: Vec<Arc<dyn Primitive>>) -> BVHAggregate {
        if std::mem::size_of::<LinearBVHNode>() > 32 {
            println!(
                "\nThe size of `LinearBVHNode` is `{}`\nBut you align it with `{}`",
                std::mem::size_of::<LinearBVHNode>(),
                32,
            );
            panic!("LinearBVHNode not aligned");
        }

        let mut bvh_primitives = vec![];
        for idx in 0..primitives.len() {
            bvh_primitives.push(BVHPrimitive::new(idx, primitives[idx].bounds()));
        }

        let mut ordered_primitives = vec![];
        let root = build_recursive(&primitives, &bvh_primitives, &mut ordered_primitives);
        bvh_primitives.clear();

        println!("BVH built (primitives: {})", ordered_primitives.len());

        let mut linear_bvh_nodes: Vec<LinearBVHNode> = vec![];
        flatten_bvh(root, &mut linear_bvh_nodes);

        return Self {
            ordered_primitives,
            linear_bvh_nodes,
        };
    }
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

        return Arc::new(build_leaf(
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

    // TODO: implement SAH
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
    // TODO: implement SAH

    let left_child = build_recursive(primitives, &left_primitives, ordered_primitives);
    left_primitives.clear();

    let right_child = build_recursive(primitives, &right_primitives, ordered_primitives);
    right_primitives.clear();

    return Arc::new(build_interior(split_axis, left_child, right_child));
}
