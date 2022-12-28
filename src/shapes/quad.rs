use crate::core::pbrt::*;

pub struct Quad {
    origin: Point,
    span0: Vector3,
    span1: Vector3,
    triangles: Vec<Arc<Triangle>>,
    // representing Quad with triangles
    // is not smart yet simple and easy
}

impl Quad {
    pub fn new(v0: Point, _span0: Vector3, _span1: Vector3) -> Self {
        let vertices = vec![v0, v0 + _span0, v0 + _span0 + _span1, v0 + _span1];
        let indices = vec![0, 2, 1, 0, 2, 3];

        let mesh = TriangleMesh::new(vertices, indices);
        let _triangles = mesh.build_triangle();

        return Self {
            origin: v0,
            span0: _span0,
            span1: _span1,
            triangles: _triangles,
        };
    }
}

impl Shape for Quad {
    fn intersect(&self, ray: &Ray, t_hit: &mut f32, interaction: &mut SurfaceInteraction) -> bool {
        for triangle in &self.triangles {
            if triangle.intersect(ray, t_hit, interaction) {
                return true;
            }
        }

        return false;
    }

    fn get_bounds(&self) -> Bounds {
        return self.triangles[0].get_bounds() + self.triangles[1].get_bounds();
    }

    fn sample(&self, sampler: &mut dyn Sampler) -> (Point, Vector3) {
        let (alpha, beta) = sampler.get_2d_sample();

        return (
            self.origin + alpha * self.span0 + beta * self.span1,
            cross(self.span0, self.span1),
        );
    }

    fn get_area(&self) -> f32 {
        return cross(self.span0, self.span1).length();
    }
}
