use crate::pbrt::*;

pub struct SimplePrimitive {
    shape: Arc<dyn Shape>,
}

impl Primitive for SimplePrimitive {
    fn intersect(&self, ray: &Ray, t_max: Float) -> Option<ShapeIntersection> {
        return self.shape.intersect(ray, t_max);
    }

    fn get_bounds(&self) -> Bounds3f {
        return self.shape.get_bounds();
    }
}

impl SimplePrimitive {
    pub fn new(shape: Arc<dyn Shape>) -> Self {
        return Self { shape };
    }
}
