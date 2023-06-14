use crate::pbrt::*;

pub struct TriangleMesh {
    pub indices: Vec<usize>,
    pub points: Vec<Point3f>,
}

impl TriangleMesh {
    pub fn new(points: Vec<Point3f>, indices: Vec<i32>) -> Self {
        if indices.len() % 3 != 0 {
            panic!("TriangleMesh: illegal parameter (indices' length can't be divided to 3)");
        }

        let indices = indices.into_iter().map(|v| v as usize).collect();
        return TriangleMesh { points, indices };
    }
}

pub struct Triangle {
    idx: usize,
    mesh: Arc<TriangleMesh>,
}

pub fn build_triangles(points: Vec<Point3f>, indices: Vec<i32>) -> Vec<Triangle> {
    let mesh = TriangleMesh::new(points, indices);
    let shared_mesh = Arc::new(mesh);

    let mut triangles: Vec<Triangle> = vec![];

    for idx in (0..shared_mesh.indices.len()).step_by(3) {
        let _triangle = Triangle::new(idx, shared_mesh.clone());
        triangles.push(_triangle);
    }

    return triangles;
}

impl Triangle {
    pub fn new(idx: usize, mesh: Arc<TriangleMesh>) -> Self {
        return Triangle { idx, mesh };
    }
}
