use std::sync::Arc;

use crate::fundamental::point::*;
use crate::ray_tracing::bounding_box::BoundingBox;
use crate::ray_tracing::intersection::*;
use crate::ray_tracing::primitive::Primitive;
use crate::ray_tracing::ray::*;

const BUCKET_NUM: usize = 12;
const EPSILON: f32 = 1.0e-6;

#[derive(Copy, Clone)]
pub struct PrimitiveInfo {
    primitive_id: usize,
    bounds: BoundingBox,
    centroid: Point,
}

impl Default for PrimitiveInfo {
    fn default() -> Self {
        Self {
            primitive_id: usize::MAX,
            bounds: BoundingBox::empty(),
            centroid: Point::new(f32::NAN, f32::NAN, f32::NAN),
        }
    }
}

impl PrimitiveInfo {
    pub fn new(id: usize, _bounds: BoundingBox, _centroid: Point) -> Self {
        Self {
            primitive_id: id,
            bounds: _bounds,
            centroid: _centroid,
        }
    }
}

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
    pub fn intersect(&self, ray: &Ray, previous_distance: f32, primitives: &Vec<Arc<dyn Primitive>>) -> Intersection {
        let (t1, t2) = self.bounds.intersect(ray);
        if t1 > t2 || t1 > previous_distance {
            return Intersection::failure();
        }

        if self.left.is_none() && self.right.is_none() {
            let mut closest_intersect = Intersection::failure();
            let mut closest_distance = previous_distance;

            for idx in self.start..self.end {
                let p = &primitives[idx];
                let intersect = p.intersect(ray, closest_distance);
                if intersect.intersected() {
                    closest_distance = intersect.distance;
                    closest_intersect = intersect;
                }
            }

            return closest_intersect;
        }

        let left_node = self.left.as_ref().unwrap();
        let right_node = self.right.as_ref().unwrap();

        let left_intersect = left_node.intersect(ray, previous_distance, primitives);
        if !left_intersect.intersected() {
            return right_node.intersect(ray, previous_distance, primitives);
        }

        let right_intersect = right_node.intersect(ray, left_intersect.distance, primitives);
        return if right_intersect.intersected() { right_intersect } else { left_intersect };
    }
}

impl Node {
    pub fn build_leaf(ordered_primitives: &mut Vec<Arc<dyn Primitive>>,
                      infos: Vec<PrimitiveInfo>,
                      primitives: &Vec<Arc<dyn Primitive>>) -> Self {
        let _start = ordered_primitives.len();
        let _end = _start + infos.len();
        let mut total_bounds = BoundingBox::empty();
        for info in &infos {
            ordered_primitives.push(primitives[info.primitive_id].clone());
            total_bounds += info.bounds;
        }

        Self {
            start: _start,
            end: _end,
            bounds: total_bounds,
            left: None,
            right: None,
        }
    }

    pub fn recursive_build(ordered_primitives: &mut Vec<Arc<dyn Primitive>>,
                           infos: Vec<PrimitiveInfo>,
                           primitives: &Vec<Arc<dyn Primitive>>) -> Self {
        let mut total_bounds = BoundingBox::empty();
        for info in &infos {
            total_bounds += info.bounds;
        }
        let total_bounds = total_bounds;

        if total_bounds.get_area() <= EPSILON {
            // when all primitives are accumulated close enough that
            // the bounding box is too small
            return Node::build_leaf(ordered_primitives, infos, primitives);
        }

        let centroids: Vec<Point> = (&infos).into_iter().map(
            |info| info.centroid).collect();
        let axis_min = min_of(&centroids);
        let axis_max = max_of(&centroids);
        let axis_extent = axis_max - axis_min;

        let num_primitives = (&infos).len();
        if num_primitives < BUCKET_NUM {
            // stop dividing primitives into buckets
            // when primitive number is too small
            let split_axis = {
                if axis_extent.x > axis_extent.y && axis_extent.x > axis_extent.z {
                    0
                } else if axis_extent.y > axis_extent.z {
                    1
                } else {
                    2
                }
            };
            let split_val = (0.5 * (axis_min + axis_max))[split_axis];

            let mut left_infos = Vec::new();
            let mut right_infos = Vec::new();
            let mut left_bounds = BoundingBox::empty();
            let mut right_bounds = BoundingBox::empty();
            for info in &infos {
                if info.centroid[split_axis] < split_val {
                    left_infos.push(*info);
                    left_bounds += info.bounds;
                } else {
                    right_infos.push(*info);
                    right_bounds += info.bounds;
                }
            }

            let split_cost = 1.0 +
                (left_bounds.get_area() * (left_infos.len() as f32) +
                    right_bounds.get_area() * (right_infos.len() as f32)) / total_bounds.get_area();

            let leaf_cost = num_primitives as f32;
            if leaf_cost <= split_cost {
                // when it cost less to aggregate all primitives into one leaf
                return Node::build_leaf(ordered_primitives, infos, primitives);
            }

            return Self {
                start: usize::MAX,
                end: usize::MAX,
                bounds: total_bounds,
                left: Some(Box::new(Node::recursive_build(ordered_primitives, left_infos, primitives))),
                right: Some(Box::new(Node::recursive_build(ordered_primitives, right_infos, primitives))),
            };
        }

        let mut ignored_axis = [false; 3];
        for axis_idx in 0..3 {
            if axis_extent[axis_idx] < 0.5 * axis_extent[(axis_idx + 1) % 3] ||
                axis_extent[axis_idx] < 0.5 * axis_extent[(axis_idx + 2) % 3] {
                // ignore this axis when it's extent is
                // comparably small compared with the other two
                ignored_axis[axis_idx] = true;
            }
        }
        let ignore_axis = ignored_axis;

        let mut min_cost = f32::INFINITY;
        let mut max_left_value = -f32::INFINITY;
        let mut split_axis = usize::MAX;

        for axis_idx in 0..3 {
            if ignore_axis[axis_idx] {
                continue;
            }

            let bucket_width = axis_extent[axis_idx] / (BUCKET_NUM as f32);

            let mut bucket_list = Vec::with_capacity(BUCKET_NUM);
            for _ in 0..BUCKET_NUM {
                bucket_list.push(Bucket::empty());
            }

            for info in &infos {
                let bucket_idx_float = (info.centroid[axis_idx] - axis_min[axis_idx]) / bucket_width;
                let bucket_idx = (bucket_idx_float as usize).max(0).min(BUCKET_NUM - 1);

                bucket_list[bucket_idx].count += 1;
                bucket_list[bucket_idx].primitive_infos.push(*info);
                bucket_list[bucket_idx].bounds += info.bounds;
            }

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

                let cost = 1.0 +
                    (left_bounds.get_area() * (left_count as f32) +
                        right_bounds.get_area() * (right_count as f32)) / total_bounds.get_area();

                if cost < min_cost {
                    min_cost = cost;
                    split_axis = axis_idx;
                    max_left_value = axis_min[axis_idx] + bucket_width * ((idx + 1) as f32);
                }
            }
        }

        let leaf_cost = num_primitives as f32;
        if leaf_cost <= min_cost {
            // when it cost less to aggregate all primitives into one leaf
            return Node::build_leaf(ordered_primitives, infos, primitives);
        }

        let mut left_infos = Vec::new();
        let mut right_infos = Vec::new();
        for info in infos {
            if info.centroid[split_axis] < max_left_value {
                left_infos.push(info);
            } else {
                right_infos.push(info);
            }
        }

        return Self {
            start: usize::MAX,
            end: usize::MAX,
            bounds: total_bounds,
            left: Some(Box::new(Node::recursive_build(ordered_primitives, left_infos, primitives))),
            right: Some(Box::new(Node::recursive_build(ordered_primitives, right_infos, primitives))),
        };
    }
}
