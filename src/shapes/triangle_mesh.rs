use crate::pbrt::*;

pub struct TriangleMesh {
    pub indices: Vec<usize>,
    pub points: Vec<Point3f>,
}

impl TriangleMesh {
    pub fn new(render_from_object: Transform, points: Vec<Point3f>, indices: Vec<usize>) -> Self {
        if indices.len() % 3 != 0 {
            panic!("TriangleMesh: illegal parameter (indices' length can't be divided to 3)");
        }

        let transformed_points = if render_from_object.is_identity() {
            points
        } else {
            points
                .into_iter()
                .map(|t| render_from_object.on_point3f(t))
                .collect()
        };

        return TriangleMesh {
            points: transformed_points,
            indices,
        };
    }

    pub fn create_triangles(self) -> Vec<Arc<Triangle>> {
        let shared_mesh = Arc::new(self);

        let mut triangles = vec![];
        for idx in (0..shared_mesh.indices.len()).step_by(3) {
            let _triangle = Arc::new(Triangle::new(idx, shared_mesh.clone()));
            triangles.push(_triangle);
        }

        return triangles;
    }
}
