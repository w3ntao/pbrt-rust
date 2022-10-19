use crate::core::interfaces::*;
use std::sync::Arc;

pub struct Triangle {
    pub origin: Point,
    pub span0: Vector3,
    pub span1: Vector3,
    pub normal: Vector3,
    bounds: Bounds,
    material: Arc<dyn Material>,
    id: u128,
}

impl Triangle {
    pub fn new(v0: Point, v1: Point, v2: Point) -> Self {
        let _span0 = v1 - v0;
        let _span1 = v2 - v0;
        return Self {
            origin: v0,
            span0: _span0,
            span1: _span1,
            normal: cross(_span0, _span1).normalize(),
            bounds: Bounds::build(&[v0, v1, v2]),
            material: Arc::new(NullMaterial {}),
            id: random_u128(),
        };
    }
}

impl Primitive for Triangle {
    fn intersect(&self, ray: &Ray, t_min: f32, t_max: f32) -> Intersection {
        let ab = cross(self.span0, self.span1);
        let det = -ab.dot(ray.d);
        if det == 0.0 {
            return Intersection::failure();
        }

        let c = ray.o - self.origin;
        let t = ab.dot(c) / det;
        if t < t_min || t > t_max {
            return Intersection::failure();
        }

        let beta = c.dot(cross(ray.d, self.span1)) / det;
        let gamma = self.span0.dot(cross(ray.d, c)) / det;
        let error_tolerance = 0.01;
        // to tolerate numerical error
        if beta < -error_tolerance
            || gamma < -error_tolerance
            || beta + gamma > 1.0 + error_tolerance
        {
            // if the intersection is outside of the triangle
            return Intersection::failure();
        }

        let cos = cosine(ray.d, self.normal);
        let normal = if cos < 0.0 { self.normal } else { -self.normal };

        return Intersection::from_outside(
            t,
            ray.get_point(t),
            normal,
            self.material.clone(),
            self.get_id(),
        );
    }

    fn get_bounds(&self) -> Bounds {
        return self.bounds;
    }

    fn set_material(&mut self, material: Arc<dyn Material>) {
        self.material = material;
    }

    fn get_id(&self) -> u128 {
        return self.id;
    }
}
