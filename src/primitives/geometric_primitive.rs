use crate::pbrt::*;

pub struct GeometricPrimitive {
    shape: Arc<dyn Shape>,
    material: Arc<dyn Material>,
    area_light: Arc<dyn Light>,
}

impl Primitive for GeometricPrimitive {
    fn intersect(&self, ray: &Ray, t_max: f64) -> Option<ShapeIntersection> {
        let mut si = match self.shape.intersect(ray, t_max) {
            None => {
                return None;
            }
            Some(_si) => _si,
        };

        si.surface_interaction
            .set_intersection_properties(self.material.clone(), Some(self.area_light.clone()));

        return Some(si);
    }

    fn fast_intersect(&self, ray: &Ray, t_max: f64) -> bool {
        return self.shape.fast_intersect(ray, t_max);
    }

    fn bounds(&self) -> Bounds3f {
        return self.shape.bounds();
    }
}

impl GeometricPrimitive {
    pub fn new(
        shape: Arc<dyn Shape>,
        material: Arc<dyn Material>,
        area_light: Arc<dyn Light>,
    ) -> Self {
        return Self {
            shape,
            material,
            area_light,
        };
    }
}
