use crate::core::pbrt::*;

pub struct Quad {
    pub origin: Point,
    pub span0: Vector3,
    pub span1: Vector3,
    pub normal: Normal,
    bounds: Bounds,
    material: Arc<dyn Material>,
    id: u128,
}

impl Quad {
    pub fn new(v0: Point, _span0: Vector3, _span1: Vector3) -> Self {
        return Self {
            origin: v0,
            span0: _span0,
            span1: _span1,
            normal: Normal::from(cross(_span0, _span1).normalize()),
            bounds: Bounds::build(&[v0, v0 + _span0, v0 + _span1, v0 + _span0 + _span1]),
            material: Arc::new(NullMaterial {}),
            id: random_u128(),
        };
    }
}

impl Primitive for Quad {
    fn intersect(&self, ray: &Ray, t_min: f32, interaction: &mut SurfaceInteraction) -> bool {
        let ab = cross(self.span0, self.span1);
        let det = -ab.dot(ray.d);
        if det == 0.0 {
            return false;
        }

        let c = ray.o - self.origin;
        let det_t = ab.dot(c);
        let t = det_t / det;
        if t < t_min || t > ray.t_max {
            return false;
        }
        let beta = c.dot(cross(ray.d, self.span1)) / det;
        let gamma = self.span0.dot(cross(ray.d, c)) / det;
        if beta < 0.0 || beta > 1.0 || gamma < 0.0 || gamma > 1.0 {
            return false;
        }

        let normal = if self.normal.dot(ray.d) < 0.0 {
            self.normal
        } else {
            -self.normal
        };
        *interaction =
            SurfaceInteraction::new(t, ray(t), normal, self.material.clone(), self.get_id());
        return true;
    }

    fn get_bounds(&self) -> Bounds {
        return self.bounds;
    }

    fn set_material(&mut self, material: Arc<dyn Material>) {
        self.material = material;
    }

    fn sample(&self) -> (Point, Vector3) {
        let alpha = random_f32(0.0, 1.0);
        let beta = random_f32(0.0, 1.0);
        return (
            self.origin + alpha * self.span0 + beta * self.span1,
            Vector3::from(self.normal),
        );
    }

    fn get_id(&self) -> u128 {
        return self.id;
    }

    fn get_area(&self) -> f32 {
        return cross(self.span0, self.span1).length();
    }
}
