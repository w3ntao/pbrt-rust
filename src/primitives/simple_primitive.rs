use crate::pbrt::*;

pub struct SimplePrimitive {
    shape: Arc<dyn Shape>,
}

impl Primitive for SimplePrimitive {
    fn intersect(&self, ray: &dyn Ray, t_max: Float) -> Option<ShapeIntersection> {
        return self.shape.intersect(ray, t_max);
    }

    fn fast_intersect(&self, ray: &dyn Ray, t_max: Float) -> bool {
        return self.shape.fast_intersect(ray, t_max);
    }

    fn bounds(&self) -> Bounds3f {
        return self.shape.bounds();
    }
}

impl SimplePrimitive {
    pub fn new(shape: Arc<dyn Shape>) -> Self {
        return Self { shape };
    }
}
