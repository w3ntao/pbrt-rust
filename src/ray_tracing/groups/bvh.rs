use std::rc::Rc;
use crate::ray_tracing::ray::*;
use crate::ray_tracing::intersection::*;
use crate::ray_tracing::primitive::Primitive;
use crate::ray_tracing::group::Group;
use crate::ray_tracing::bounding_box::BoundingBox;

use crate::ray_tracing::groups::bvh_node::{Node, PrimitiveInfo};

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
        return self.root.as_ref().unwrap().intersect(ray, previous_distance, &self.primitives);
    }

    fn get_bounds(&self) -> BoundingBox {
        return self.bounds;
    }
}

impl BVH {
    pub fn build_index(&mut self) {
        println!("Building BVH");
        let mut primitive_infos = vec![PrimitiveInfo::default(); self.primitives.len()];
        for idx in 0..self.primitives.len() {
            let bounds = self.primitives[idx].get_bounds();
            let centroid = 0.5 * (bounds.min + bounds.max);
            primitive_infos[idx] = PrimitiveInfo::new(idx, bounds, centroid);
        }

        let mut ordered_primitives = Vec::<Rc<dyn Primitive>>::new();
        self.root = Some(Box::new(Node::recursive_build(&mut ordered_primitives,
                                                        primitive_infos,
                                                        &self.primitives)));
        self.primitives = ordered_primitives;
        println!("BVH built");
    }
}
