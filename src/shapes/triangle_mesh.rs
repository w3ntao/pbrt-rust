use crate::pbrt::*;

pub struct TriangleMesh {
    pub reverse_orientation: bool,
    pub indices: Vec<usize>,
    pub points: Vec<Point3f>,
    pub normals: Vec<Normal3f>,
    pub uv: Vec<Point2f>,
}

impl TriangleMesh {
    pub fn new(
        render_from_object: &Transform,
        reverse_orientation: bool,
        points: Vec<Point3f>,
        indices: Vec<usize>,
        normals: Vec<Normal3f>,
        uv: Vec<Point2f>,
    ) -> Self {
        if indices.len() % 3 != 0 {
            panic!("TriangleMesh: illegal parameter (indices' length can't be divided to 3)");
        }

        let (transformed_points, transformed_normals) = if render_from_object.is_identity() {
            (points, normals)
        } else {
            (
                points
                    .into_par_iter()
                    .map(|x| render_from_object.on_point3f(x))
                    .collect(),
                normals
                    .into_par_iter()
                    .map(|x| render_from_object.on_normal3f(x))
                    .collect(),
            )
        };

        return TriangleMesh {
            reverse_orientation,
            points: transformed_points,
            indices,
            normals: transformed_normals,
            uv,
        };
    }

    pub fn create_triangles(self) -> Vec<Arc<dyn Shape>> {
        let shared_mesh = Arc::new(self);

        let mut triangles: Vec<Arc<dyn Shape>> = vec![];
        for idx in (0..shared_mesh.indices.len()).step_by(3) {
            let _triangle = Arc::new(Triangle::new(idx, shared_mesh.clone()));
            triangles.push(_triangle);
        }

        return triangles;
    }
}
