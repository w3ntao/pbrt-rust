use crate::core::pbrt::*;

pub struct Quad {
    pub origin: Point,
    pub span0: Vector3,
    pub span1: Vector3,
    material: Arc<dyn Material>,
    pub triangles: Vec<Arc<Triangle>>,
    // TODO: rewrite Quad, especially intersect()
}

impl Quad {
    pub fn new(v0: Point, _span0: Vector3, _span1: Vector3) -> Self {
        let vertices = vec![v0, v0 + _span0, v0 + _span1, v0 + _span0 + _span1];
        let indices = vec![0, 1, 2, 1, 2, 3];

        let mesh = TriangleMesh::new(vertices, indices);
        let _triangles = mesh.build_triangle();

        return Self {
            origin: v0,
            span0: _span0,
            span1: _span1,
            material: Arc::new(NullMaterial {}),
            triangles: _triangles,
        };
    }
}

impl Shape for Quad {
    fn intersect(&self, ray: &Ray, interaction: &mut SurfaceInteraction) -> bool {
        for triangle in &self.triangles {
            if triangle.intersect(ray, interaction) {
                interaction.material = self.material.clone();
                return true;
            }
        }

        return false;
    }

    fn get_bounds(&self) -> Bounds {
        return self.triangles[0].get_bounds() + self.triangles[1].get_bounds();
    }

    fn set_material(&mut self, material: Arc<dyn Material>) {
        self.material = material;
    }

    fn sample(&self) -> (Point, Vector3) {
        let alpha = random_f32(0.0, 1.0);
        let beta = random_f32(0.0, 1.0);

        return (
            self.origin + alpha * self.span0 + beta * self.span1,
            cross(self.span0, self.span1),
        );
    }

    fn get_area(&self) -> f32 {
        return cross(self.span0, self.span1).length();
    }
}
