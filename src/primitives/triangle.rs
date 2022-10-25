use crate::core::pbrt::*;

pub struct TriangleMesh {
    vertices: Vec<Point>,
    indices: Vec<usize>,
}

impl TriangleMesh {
    pub fn new(_vertices: Vec<Point>, _indices: Vec<usize>) -> Self {
        return Self {
            vertices: _vertices,
            indices: _indices,
        };
    }

    pub fn build_triangle(self) -> Vec<Arc<Triangle>> {
        let mut triangles = Vec::new();
        let arc_self = Arc::new(self);
        for index in (0..arc_self.indices.len()).step_by(3) {
            triangles.push(Arc::new(Triangle::new(index, arc_self.clone())));
        }
        return triangles;
    }
}

pub struct Triangle {
    mesh_index: usize,
    mesh_root: Arc<TriangleMesh>,
    material: Arc<dyn Material>,
    object_id: u128,
}

impl Triangle {
    pub fn new(_index: usize, _mesh: Arc<TriangleMesh>) -> Self {
        return Self {
            mesh_index: _index,
            material: Arc::new(NullMaterial {}),
            mesh_root: _mesh,
            object_id: random_u128(),
        };
    }
}

impl Primitive for Triangle {
    fn intersect(&self, ray: &Ray, t_min: f32, t_max: f32) -> SurfaceInteraction {
        let vertex_idx0 = self.mesh_root.indices[self.mesh_index];
        let vertex_idx1 = self.mesh_root.indices[self.mesh_index + 1];
        let vertex_idx2 = self.mesh_root.indices[self.mesh_index + 2];
        let p0 = self.mesh_root.vertices[vertex_idx0];
        let p1 = self.mesh_root.vertices[vertex_idx1];
        let p2 = self.mesh_root.vertices[vertex_idx2];

        let span0 = p1 - p0;
        let span1 = p2 - p0;
        let normal = Normal::from(cross(span0, span1).normalize());

        let ab = cross(span0, span1);
        let det = -ab.dot(ray.d);
        if det == 0.0 {
            return SurfaceInteraction::failure();
        }

        let c = ray.o - p0;
        let t = ab.dot(c) / det;
        if t < t_min || t > t_max {
            return SurfaceInteraction::failure();
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
            return SurfaceInteraction::failure();
        }

        let cos = normal.cosine(ray.d);
        let normal = if cos < 0.0 { normal } else { -normal };

        return SurfaceInteraction::new(t, ray(t), normal, self.material.clone(), self.get_id());
    }

    fn get_bounds(&self) -> Bounds {
        let vertex_idx0 = self.mesh_root.indices[self.mesh_index];
        let vertex_idx1 = self.mesh_root.indices[self.mesh_index + 1];
        let vertex_idx2 = self.mesh_root.indices[self.mesh_index + 2];
        let p0 = self.mesh_root.vertices[vertex_idx0];
        let p1 = self.mesh_root.vertices[vertex_idx1];
        let p2 = self.mesh_root.vertices[vertex_idx2];

        return Bounds::build(&[p0, p1, p2]);
    }

    fn set_material(&mut self, material: Arc<dyn Material>) {
        self.material = material;
    }

    fn get_id(&self) -> u128 {
        return self.object_id;
    }
}
