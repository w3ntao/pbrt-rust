use std::rc::Rc;
use crate::fundamental::point::*;
use crate::ray_tracing::ray::*;
use crate::ray_tracing::intersection::*;
use crate::ray_tracing::primitive::Primitive;
use crate::ray_tracing::group::Group;
use crate::ray_tracing::bounding_box::BoundingBox;

struct Node {
    primitives: Vec<Rc<dyn Primitive>>,
    bounds: BoundingBox,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

impl Primitive for Node {
    fn intersect(&self, ray: Rc<Ray>, previous_distance: f32) -> Intersection {
        let (t1, t2) = self.bounds.intersect(Rc::clone(&ray));
        if t1 > t2 || t1 > previous_distance {
            return Intersection::failure();
        }

        if self.left.is_none() && self.right.is_none() {
            let mut closest_intersect = Intersection::failure();
            let mut closest_distance = previous_distance;

            for p in &self.primitives {
                let intersect = p.intersect(Rc::clone(&ray), closest_distance);
                if intersect.intersected() {
                    closest_distance = intersect.distance;
                    closest_intersect = intersect;
                }
            }
            return closest_intersect;
        }

        let left_intersect = self.left.as_ref().unwrap().intersect(Rc::clone(&ray), previous_distance);
        let right_intersect = self.right.as_ref().unwrap().intersect(Rc::clone(&ray), left_intersect.distance);

        return {
            if left_intersect.distance < right_intersect.distance {
                left_intersect
            } else {
                right_intersect
            }
        };
    }

    fn get_bounds(&self) -> BoundingBox {
        return self.bounds;
    }
}

impl Node {
    fn new(_primitives: Vec<Rc<dyn Primitive>>) -> Self {
        let mut total_bounds = BoundingBox::empty();
        for p in &_primitives {
            total_bounds += p.get_bounds();
        }
        let total_bounds = total_bounds;

        if _primitives.len() <= 20 {
            return Self {
                primitives: _primitives,
                bounds: total_bounds,
                left: None,
                right: None,
            };
        }

        let centroids: Vec<Point> = (&_primitives).into_iter().map(
            |p| p.get_bounds().get_centroid()).collect();
        let extent = max_of(&centroids) - min_of(&centroids);
        let axis = {
            if extent[0] >= extent[1] && extent[0] >= extent[2] {
                0
            } else if extent[1] >= extent[2] {
                1
            } else {
                2
            }
        };

        let mut sorted_primitives = _primitives;
        sorted_primitives.sort_by(|a, b|
            a.get_bounds().get_centroid()[axis].partial_cmp(
                &b.get_bounds().get_centroid()[axis]).unwrap());

        let mid = sorted_primitives.len() / 2;
        let left_nodes = &sorted_primitives[0..mid];
        let right_nodes = &sorted_primitives[mid..sorted_primitives.len()];

        return Self {
            primitives: Vec::default(),
            bounds: total_bounds,
            left: Some(Box::new(Node::new(left_nodes.to_vec()))),
            right: Some(Box::new(Node::new(right_nodes.to_vec()))),
        };
    }
}

pub struct BVH {
    primitives: Vec<Rc<dyn Primitive>>,
    bounds: BoundingBox,
    root: Option<Box<Node>>,
}

impl Default for BVH {
    fn default() -> Self {
        BVH {
            primitives: Vec::default(),
            bounds: BoundingBox::empty(),
            root: None,
        }
    }
}

impl Group for BVH {
    fn add(&mut self, p: Rc<dyn Primitive>) {
        self.bounds += p.get_bounds();
        self.primitives.push(p);
    }
}

impl Primitive for BVH {
    fn intersect(&self, ray: Rc<Ray>, previous_distance: f32) -> Intersection {
        return self.root.as_ref().unwrap().intersect(ray, previous_distance);
    }

    fn get_bounds(&self) -> BoundingBox {
        return self.bounds;
    }
}

impl BVH {
    pub fn build_index(&mut self) {
        println!("Building BVH");
        self.root = Some(Box::new(Node::new(self.primitives.clone())));
        println!("BVH built");
    }
}
