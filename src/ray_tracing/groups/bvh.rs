use std::sync::Arc;
use std::time::Instant;

use crate::ray_tracing::bounding_box::BoundingBox;
use crate::ray_tracing::group::Group;
use crate::ray_tracing::groups::bvh_node::{Node, PrimitiveInfo};
use crate::ray_tracing::intersection::*;
use crate::ray_tracing::material::Material;
use crate::ray_tracing::primitive::Primitive;
use crate::ray_tracing::ray::*;

pub struct BVH {
    primitives: Vec<Arc<dyn Primitive>>,
    bounds: BoundingBox,
    root: Option<Box<Node>>,
    index_built: bool,
}

impl Default for BVH {
    fn default() -> Self {
        BVH {
            primitives: Vec::default(),
            bounds: BoundingBox::empty(),
            root: None,
            index_built: false,
        }
    }
}

impl Group for BVH {
    fn add(&mut self, p: Arc<dyn Primitive>) {
        self.bounds += p.get_bounds();
        self.primitives.push(p);
        self.index_built = false;
    }
}

impl Primitive for BVH {
    fn intersect(&self, ray: &Ray, t_min: f32, t_max: f32) -> Intersection {
        if !self.index_built {
            panic!("BVH: You should invoke function `build_index()` before intersect with it")
        }

        return self.root.as_ref().unwrap().intersect(ray, t_min, t_max, &self.primitives);
    }

    fn get_bounds(&self) -> BoundingBox {
        return self.bounds;
    }

    fn set_material(&mut self, _: Arc<dyn Material>) {
        panic!("You shouldn't invoke function `set_material()` from BVH")
    }
}

impl BVH {
    pub fn build_index(&mut self) {
        let start = Instant::now();
        let mut primitive_infos = vec![PrimitiveInfo::default(); self.primitives.len()];
        for idx in 0..self.primitives.len() {
            let bounds = self.primitives[idx].get_bounds();
            let centroid = 0.5 * (bounds.min + bounds.max);
            primitive_infos[idx] = PrimitiveInfo::new(idx, bounds, centroid);
        }

        let mut ordered_primitives = Vec::<Arc<dyn Primitive>>::new();
        self.root = Some(Box::new(Node::recursive_build(&mut ordered_primitives,
                                                        primitive_infos,
                                                        &self.primitives)));
        self.primitives = ordered_primitives;
        println!("BVH building took {:.2}s", start.elapsed().as_secs_f32());
        self.index_built = true;
    }
}
