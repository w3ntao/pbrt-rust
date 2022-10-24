use crate::core::pbrt::*;

pub struct Triangle {
    p0: Point,
    p1: Point,
    p2: Point,
    material: Arc<dyn Material>,
    id: u128,
}

impl Triangle {
    pub fn new(v0: Point, v1: Point, v2: Point) -> Self {
        return Self {
            p0: v0,
            p1: v1,
            p2: v2,
            material: Arc::new(NullMaterial {}),
            id: random_u128(),
        };
    }
}

impl Primitive for Triangle {
    fn intersect(&self, ray: &Ray, t_min: f32, t_max: f32) -> Intersection {
        let span0 = self.p1 - self.p0;
        let span1 = self.p2 - self.p0;
        let normal = Normal::from(cross(span0, span1).normalize());

        let ab = cross(span0, span1);
        let det = -ab.dot(ray.d);
        if det == 0.0 {
            return Intersection::failure();
        }

        let c = ray.o - self.p0;
        let t = ab.dot(c) / det;
        if t < t_min || t > t_max {
            return Intersection::failure();
        }

        let beta = c.dot(cross(ray.d, span1)) / det;
        let gamma = span0.dot(cross(ray.d, c)) / det;
        let error_tolerance = 0.01;
        // to tolerate numerical error
        if beta < -error_tolerance
            || gamma < -error_tolerance
            || beta + gamma > 1.0 + error_tolerance
        {
            // if the intersection is outside of the triangle
            return Intersection::failure();
        }

        let cos = normal.cosine(ray.d);
        let normal = if cos < 0.0 { normal } else { -normal };

        return Intersection::from_outside(t, ray(t), normal, self.material.clone(), self.get_id());
    }

    fn get_bounds(&self) -> Bounds {
        return Bounds::build(&[self.p0, self.p1, self.p2]);
    }

    fn set_material(&mut self, material: Arc<dyn Material>) {
        self.material = material;
    }

    fn get_id(&self) -> u128 {
        return self.id;
    }
}
