use crate::accelerators::bvh_node::{Node, PrimitiveInfo};
use crate::core::pbrt::*;

pub struct BVH {
    primitives: Vec<Arc<dyn Primitive>>,
    bounds: Bounds,
    root: Option<Box<Node>>,
    index_built: bool,
}

impl Default for BVH {
    fn default() -> Self {
        BVH {
            primitives: Vec::default(),
            bounds: Bounds::empty(),
            root: None,
            index_built: false,
        }
    }
}

impl Aggregate for BVH {
    fn add(&mut self, p: Arc<dyn Primitive>) {
        self.bounds += p.get_bounds();
        self.primitives.push(p);
        self.index_built = false;
    }
}

impl Primitive for BVH {
    fn intersect(&self, ray: &Ray, t_min: f32, interaction: &mut SurfaceInteraction) -> bool {
        if !self.index_built {
            panic!("BVH: You should invoke function `build_index()` before intersect with it")
        }

        return self
            .root
            .as_ref()
            .unwrap()
            .intersect(ray, t_min, &self.primitives, interaction);
    }

    fn get_bounds(&self) -> Bounds {
        return self.bounds;
    }

    fn set_material(&mut self, _: Arc<dyn Material>) {
        panic!("You shouldn't invoke function `set_material()` from BVH")
    }

    fn sample(&self) -> (Point, Vector3) {
        panic!("sample() is not implemented for BVH");
    }

    fn get_id(&self) -> u128 {
        panic!("get_id() is not implemented for BVH");
    }

    fn get_area(&self) -> f32 {
        panic!("get_area() is not implemented for BVH");
    }
}

impl BVH {
    pub fn build_index(&mut self) {
        let start = Instant::now();
        let mut primitive_infos = vec![PrimitiveInfo::default(); self.primitives.len()];
        for idx in 0..self.primitives.len() {
            let bounds = self.primitives[idx].get_bounds();
            let centroid = 0.5 * (bounds.p_min + bounds.p_max);
            primitive_infos[idx] = PrimitiveInfo::new(idx, bounds, centroid);
        }

        let mut ordered_primitives = Vec::<Arc<dyn Primitive>>::new();
        self.root = Some(Box::new(Node::recursive_build(
            &mut ordered_primitives,
            primitive_infos,
            &self.primitives,
        )));
        self.primitives = ordered_primitives;
        println!("BVH building took {:.2}s", start.elapsed().as_secs_f32());
        self.index_built = true;
    }
}
