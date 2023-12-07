use crate::pbrt::*;

pub struct SimplePrimitive {
    shape: Arc<dyn Shape>,
    material: Arc<dyn Material>,
}

impl Primitive for SimplePrimitive {
    fn intersect(&self, ray: &Ray, t_max: Float) -> Option<ShapeIntersection> {
        let mut si = match self.shape.intersect(ray, t_max) {
            None => {
                return None;
            }
            Some(_si) => _si,
        };

        si.surface_interaction
            .set_intersection_properties(self.material.clone(), None);

        return Some(si);
    }

    fn fast_intersect(&self, ray: &Ray, t_max: Float) -> bool {
        return self.shape.fast_intersect(ray, t_max);
    }

    fn bounds(&self) -> Bounds3f {
        return self.shape.bounds();
    }
}

impl SimplePrimitive {
    pub fn new(shape: Arc<dyn Shape>, material: Arc<dyn Material>) -> Self {
        return Self { shape, material };
    }
}
