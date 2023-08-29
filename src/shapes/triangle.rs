use crate::pbrt::*;

const TYPE_TESTER: Float = Float::NAN;

#[derive(Clone)]
pub struct Triangle {
    idx: usize,
    mesh: Arc<TriangleMesh>,
}

struct TriangleIntersection {
    pub b0: Float,
    pub b1: Float,
    pub b2: Float,
    pub t: Float,
}

fn intersect_triangle(
    ray: &Ray,
    t_max: Float,
    p0: Point3f,
    p1: Point3f,
    p2: Point3f,
) -> Option<TriangleIntersection> {
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

    let mut e0 = difference_of_products(p1t.x, p2t.y, p1t.y, p2t.x);
    let mut e1 = difference_of_products(p2t.x, p0t.y, p2t.y, p0t.x);
    let mut e2 = difference_of_products(p0t.x, p1t.y, p0t.y, p1t.x);

    if type_of(TYPE_TESTER) == "f32" && (e0 == 0.0 || e1 == 0.0 || e2 == 0.0) {
        let p2txp1ty = p2t.x as f64 * p1t.y as f64;
        let p2typ1tx = p2t.y as f64 * p1t.x as f64;
        e0 = (p2typ1tx - p2txp1ty) as Float;

        let p0txp2ty = p0t.x as f64 * p2t.y as f64;
        let p0typ2tx = p0t.y as f64 * p2t.x as f64;
        e1 = (p0typ2tx - p0txp2ty) as Float;

        let p1txp0ty = p1t.x as f64 * p0t.y as f64;
        let p1typ0tx = p1t.y as f64 * p0t.x as f64;
        e2 = (p1typ0tx - p1txp0ty) as Float;
    }

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

    // Compute barycentric coordinates and $t$ value for triangle intersection
    let invDet = 1.0 / det;
    let b0 = e0 * invDet;
    let b1 = e1 * invDet;
    let b2 = e2 * invDet;
    let t = tScaled * invDet;

    // Ensure that computed triangle $t$ is conservatively greater than zero
    // Compute $\delta_z$ term for triangle $t$ error bounds
    let maxZt = Vector3f::new(p0t.z, p1t.z, p2t.z)
        .abs()
        .max_component_value();

    let deltaZ = gamma(3) * maxZt;

    // Compute $\delta_x$ and $\delta_y$ terms for triangle $t$ error bounds
    let maxXt = Vector3f::new(p0t.x, p1t.x, p2t.x)
        .abs()
        .max_component_value();

    let maxYt = Vector3f::new(p0t.y, p1t.y, p2t.y)
        .abs()
        .max_component_value();

    let deltaX = gamma(5) * (maxXt + maxZt);
    let deltaY = gamma(5) * (maxYt + maxZt);

    // Compute $\delta_e$ term for triangle $t$ error bounds
    let deltaE = 2.0 * (gamma(2) * maxXt * maxYt + deltaY * maxXt + deltaX * maxYt);

    // Compute $\delta_t$ term for triangle $t$ error bounds and check _t_
    let maxE = Vector3f::new(e0, e1, e2).abs().max_component_value();

    let deltaT = 3.0 * (gamma(3) * maxE * maxZt + deltaE * maxZt + deltaZ * maxE) * invDet.abs();

    if t <= deltaT {
        return None;
    }

    return Some(TriangleIntersection { b0, b1, b2, t });
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

    fn build_interaction(&self, ti: &TriangleIntersection, wo: Vector3f) -> SurfaceInteraction {
        let (p0, p1, p2) = self.get_points();
        let dp02 = p0 - p2;
        let dp12 = p1 - p2;

        // Interpolate $(u,v)$ parametric coordinates and hit point
        let pHit = ti.b0 * p0 + ti.b1 * p1 + ti.b2 * p2;
        let pAbsSum = (ti.b0 * p0).abs() + (ti.b1 * p1).abs() + (ti.b2 * p2).abs();

        let pError = gamma(7) * Vector3f::from(pAbsSum);

        let n = Normal3f::from(dp02.cross(dp12).normalize());
        // TODO: flip n with `reverseOrientation` and transformSwapsHandedness?

        return SurfaceInteraction {
            pi: Point3fi::from_value_and_error(pHit, pError),
            n,
            wo,
        };
    }
}

impl Shape for Triangle {
    fn intersect(&self, ray: &Ray, t_max: Float) -> Option<ShapeIntersection> {
        let (p0, p1, p2) = self.get_points();

        let triangle_intersection = match intersect_triangle(ray, t_max, p0, p1, p2) {
            None => {
                return None;
            }
            Some(_ti) => _ti,
        };

        return Some(ShapeIntersection {
            t_hit: triangle_intersection.t,
            interaction: self.build_interaction(&triangle_intersection, -ray.d),
        });
    }

    fn fast_intersect(&self, ray: &Ray, t_max: Float) -> bool {
        let (p0, p1, p2) = self.get_points();
        return intersect_triangle(ray, t_max, p0, p1, p2).is_some();
    }

    fn bounds(&self) -> Bounds3f {
        let (p0, p1, p2) = self.get_points();
        return Bounds3f::from_multiple_points(&[p0, p1, p2]);
    }
}
