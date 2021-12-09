use std::rc::Rc;
use crate::fundamental::point::*;
use crate::ray_tracing::ray::*;
use crate::ray_tracing::intersection::*;
use crate::ray_tracing::primitive::Primitive;
use crate::ray_tracing::bounding_box::BoundingBox;

#[derive(Copy, Clone)]
pub struct PrimitiveInfo {
    id: usize,
    bounds: BoundingBox,
    centroid: Point,
}

impl Default for PrimitiveInfo {
    fn default() -> Self {
        Self {
            id: usize::MAX,
            bounds: BoundingBox::empty(),
            centroid: Point::new(f32::NAN, f32::NAN, f32::NAN),
        }
    }
}

impl PrimitiveInfo {
    pub fn new(_id: usize, _bounds: BoundingBox, _centroid: Point) -> Self {
        Self {
            id: _id,
            bounds: _bounds,
            centroid: _centroid,
        }
    }
}

const BUCKET_NUM: usize = 12;

struct Bucket {
    count: i32,
    bounds: BoundingBox,
    primitive_infos: Vec<PrimitiveInfo>,
}

impl Bucket {
    fn empty() -> Self {
        Self {
            count: 0,
            bounds: BoundingBox::empty(),
            primitive_infos: vec![],
        }
    }
}

pub struct Node {
    start: usize,
    end: usize,
    bounds: BoundingBox,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

impl Node {
    pub fn intersect(&self, ray: Rc<Ray>, previous_distance: f32, primitives: &Vec<Rc<dyn Primitive>>) -> Intersection {
        let (t1, t2) = self.bounds.intersect(Rc::clone(&ray));
        if t1 > t2 || t1 > previous_distance {
            return Intersection::failure();
        }

        if self.left.is_none() && self.right.is_none() {
            let mut closest_intersect = Intersection::failure();
            let mut closest_distance = previous_distance;

            for idx in self.start..self.end {
                let p = &primitives[idx];
                let intersect = p.intersect(Rc::clone(&ray), closest_distance);
                if intersect.intersected() {
                    closest_distance = intersect.distance;
                    closest_intersect = intersect;
                }
            }

            return closest_intersect;
        }

        let left_node = self.left.as_ref().unwrap();
        let right_node = self.right.as_ref().unwrap();

        let left_intersect = left_node.intersect(Rc::clone(&ray), previous_distance, primitives);
        let right_intersect = right_node.intersect(Rc::clone(&ray), left_intersect.distance, primitives);

        return {
            if left_intersect.distance < right_intersect.distance {
                left_intersect
            } else {
                right_intersect
            }
        };
    }
}

impl Node {
    pub fn build_leaf(ordered_primitives: &mut Vec<Rc<dyn Primitive>>,
                      primitive_info: Vec<PrimitiveInfo>,
                      _primitives: &Vec<Rc<dyn Primitive>>) -> Self {
        let mut total_bounds = BoundingBox::empty();
        let _start = ordered_primitives.len();
        for info in primitive_info {
            total_bounds += info.bounds;
            ordered_primitives.push(_primitives[info.id].clone());
        }
        let _end = ordered_primitives.len();

        Self {
            start: _start,
            end: _end,
            bounds: total_bounds,
            left: None,
            right: None,
        }
    }

    pub fn recursive_build(ordered_primitives: &mut Vec<Rc<dyn Primitive>>,
                           primitive_infos: Vec<PrimitiveInfo>,
                           _primitives: &Vec<Rc<dyn Primitive>>) -> Self {
        if primitive_infos.len() < BUCKET_NUM {
            return Node::build_leaf(ordered_primitives, primitive_infos, _primitives);
        }

        let mut total_bounds = BoundingBox::empty();
        for info in &primitive_infos {
            total_bounds += info.bounds;
        }
        let total_bounds = total_bounds;

        let centroids: Vec<Point> = (&primitive_infos).into_iter().map(
            |info| info.centroid).collect();
        let axis_min = min_of(&centroids);
        let axis_max = max_of(&centroids);
        let axis_extent = axis_max - axis_min;

        let mut min_cost = f32::INFINITY;
        let mut max_left_value = -f32::INFINITY;
        let mut split_axis = usize::MAX;

        for axis_idx in 0..3 {
            let bucket_width = axis_extent[axis_idx] / (BUCKET_NUM as f32);

            let mut bucket_list = Vec::with_capacity(BUCKET_NUM);
            for _ in 0..BUCKET_NUM {
                bucket_list.push(Bucket::empty());
            }

            for info in &primitive_infos {
                let bucket_idx_float = (info.centroid[axis_idx] - axis_min[axis_idx]) / bucket_width;
                let bucket_idx = (bucket_idx_float as usize).max(0).min(BUCKET_NUM - 1);

                bucket_list[bucket_idx].count += 1;
                bucket_list[bucket_idx].primitive_infos.push(info.clone());
                bucket_list[bucket_idx].bounds += info.bounds;
            }

            let mut cost = [0.0; BUCKET_NUM - 1];
            for idx in 0..BUCKET_NUM - 1 {
                let mut left_bounds = BoundingBox::empty();
                let mut right_bounds = BoundingBox::empty();

                let mut left_count = 0;
                let mut right_count = 0;
                for left_idx in 0..idx + 1 {
                    left_bounds += bucket_list[left_idx].bounds;
                    left_count += bucket_list[left_idx].count;
                }
                for right_idx in idx + 1..BUCKET_NUM {
                    right_bounds += bucket_list[right_idx].bounds;
                    right_count += bucket_list[right_idx].count;
                }

                cost[idx] = 1.0 +
                    (left_bounds.get_area() * (left_count as f32) +
                        right_bounds.get_area() * (right_count as f32)) / total_bounds.get_area();
            }

            for bucket_idx in 0..BUCKET_NUM - 1 {
                if bucket_list[bucket_idx].primitive_infos.is_empty() { continue; }

                if cost[bucket_idx] < min_cost {
                    min_cost = cost[bucket_idx];
                    split_axis = axis_idx;
                    max_left_value = axis_min[axis_idx] + bucket_width * ((bucket_idx + 1) as f32);
                }
            }
        }

        let leaf_cost = primitive_infos.len() as f32;
        if leaf_cost <= min_cost {
            return Node::build_leaf(ordered_primitives, primitive_infos, _primitives);
        }

        let mut left_infos = Vec::new();
        let mut right_infos = Vec::new();
        for info in primitive_infos {
            if info.centroid[split_axis] < max_left_value {
                left_infos.push(info.clone());
            } else {
                right_infos.push(info.clone());
            }
        }

        return Self {
            start: usize::MAX,
            end: usize::MAX,
            bounds: total_bounds,
            left: Some(Box::new(Node::recursive_build(ordered_primitives, left_infos, _primitives))),
            right: Some(Box::new(Node::recursive_build(ordered_primitives, right_infos, _primitives))),
        };
    }
}
