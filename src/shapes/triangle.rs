use crate::pbrt::*;

pub struct TriangleMesh {
    pub indices: Vec<usize>,
    pub points: Vec<Point3f>,
}

impl TriangleMesh {
    pub fn new(renderFromObject: Transform, points: Vec<Point3f>, indices: Vec<usize>) -> Self {
        if indices.len() % 3 != 0 {
            panic!("TriangleMesh: illegal parameter (indices' length can't be divided to 3)");
        }

        let points = if renderFromObject.is_identity() {
            points
        } else {
            points
                .into_iter()
                .map(|t| renderFromObject.on_point3f(t))
                .collect()
        };

        return TriangleMesh { points, indices };
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

#[derive(Clone)]
pub struct Triangle {
    idx: usize,
    mesh: Arc<TriangleMesh>,
}

fn intersect_triangle(
    ray: &Ray,
    t_max: Float,
    p0: Point3f,
    p1: Point3f,
    p2: Point3f,
) -> Option<ShapeIntersection> {
    let normal = (p2 - p0).cross(p1 - p0);

    if normal.length_squared() == 0.0 {
        return None;
    }

    let ray_origin_v3 = Vector3f::from(ray.o);

    let p0t = p0 - ray_origin_v3;
    let p1t = p1 - ray_origin_v3;
    let p2t = p2 - ray_origin_v3;

    let kz = ray.d.abs().max_component_index();
    let kx = (kz + 1) % 3;
    let ky = (kx + 1) % 3;

    let permuted_idx = [kx, ky, kz];
    let d = ray.d.permute(permuted_idx);

    let mut p0t = p0t.permute(permuted_idx);
    let mut p1t = p1t.permute(permuted_idx);
    let mut p2t = p2t.permute(permuted_idx);

    let Sx = -d.x / d.z;
    let Sy = -d.y / d.z;
    let Sz = 1.0 / d.z;

    p0t.x += Sx * p0t.z;
    p0t.y += Sy * p0t.z;
    p1t.x += Sx * p1t.z;
    p1t.y += Sy * p1t.z;
    p2t.x += Sx * p2t.z;
    p2t.y += Sy * p2t.z;

    let e0 = difference_of_products(p1t.x, p2t.y, p1t.y, p2t.x);
    let e1 = difference_of_products(p2t.x, p0t.y, p2t.y, p0t.x);
    let e2 = difference_of_products(p0t.x, p1t.y, p0t.y, p1t.x);

    //TODO: PBRT-v4 shapes.cpp line 217

    if (e0 < 0.0 || e1 < 0.0 || e2 < 0.0) && (e0 > 0.0 || e1 > 0.0 || e2 > 0.0) {
        return None;
    }

    let det = e0 + e1 + e2;
    if det == 0.0 {
        return None;
    }

    // Compute scaled hit distance to triangle and test against ray $t$ range
    p0t.z *= Sz;
    p1t.z *= Sz;
    p2t.z *= Sz;

    let tScaled = e0 * p0t.z + e1 * p1t.z + e2 * p2t.z;
    if det < 0.0 && (tScaled >= 0.0 || tScaled < t_max * det) {
        return None;
    }

    if det > 0.0 && (tScaled <= 0.0 || tScaled > t_max * det) {
        return None;
    }

    return Some(ShapeIntersection {
        normal: Normal3f::from(
            if normal.dot(ray.d) > 0.0 {
                -normal
            } else {
                normal
            }
            .normalize(),
        ),
    });

    // TODO: Intersection computation not finished
}

impl Triangle {
    pub fn new(idx: usize, mesh: Arc<TriangleMesh>) -> Self {
        return Triangle { idx, mesh };
    }

    fn get_points(&self) -> (Point3f, Point3f, Point3f) {
        let v0 = self.mesh.indices[self.idx + 0];
        let v1 = self.mesh.indices[self.idx + 1];
        let v2 = self.mesh.indices[self.idx + 2];

        return (
            self.mesh.points[v0],
            self.mesh.points[v1],
            self.mesh.points[v2],
        );
    }
}

impl Shape for Triangle {
    fn intersect(&self, ray: &Ray, t_max: Float) -> Option<ShapeIntersection> {
        let (p0, p1, p2) = self.get_points();

        return intersect_triangle(ray, t_max, p0, p1, p2);
    }

    fn get_bounds(&self) -> Bounds3f {
        let (p0, p1, p2) = self.get_points();
        return Bounds3f::from_multiple_points(&[p0, p1, p2]);
    }
}
