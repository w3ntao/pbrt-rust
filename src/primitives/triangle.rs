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
}

impl Triangle {
    pub fn new(_index: usize, _mesh: Arc<TriangleMesh>) -> Self {
        return Self {
            mesh_index: _index,
            material: Arc::new(NullMaterial {}),
            mesh_root: _mesh,
        };
    }
}

impl Primitive for Triangle {
    fn intersect(&self, ray: &Ray, interaction: &mut SurfaceInteraction) -> bool {
        let vertex_idx0 = self.mesh_root.indices[self.mesh_index];
        let vertex_idx1 = self.mesh_root.indices[self.mesh_index + 1];
        let vertex_idx2 = self.mesh_root.indices[self.mesh_index + 2];
        let p0 = self.mesh_root.vertices[vertex_idx0];
        let p1 = self.mesh_root.vertices[vertex_idx1];
        let p2 = self.mesh_root.vertices[vertex_idx2];

        // from pbrt: https://www.pbr-book.org/3ed-2018/Shapes/Triangle_Meshes
        // Perform ray--triangle intersection test
        // Transform triangle vertices to ray coordinate space
        // Translate vertices based on ray origin
        let p0t = Point::from(p0 - ray.o);
        let p1t = Point::from(p1 - ray.o);
        let p2t = Point::from(p2 - ray.o);

        let kz = ray.d.abs().max_dimension();
        let kx = (kz + 1) % 3;
        let ky = (kx + 1) % 3;

        let d = ray.d.permute(kx, ky, kz);

        let mut p0t = p0t.permute(kx, ky, kz);
        let mut p1t = p1t.permute(kx, ky, kz);
        let mut p2t = p2t.permute(kx, ky, kz);

        // Apply shear transformation to translated vertex positions

        let Sx = -d.x / d.z;
        let Sy = -d.y / d.z;
        let Sz = 1.0 / d.z;

        p0t.x += Sx * p0t.z;
        p0t.y += Sy * p0t.z;
        p1t.x += Sx * p1t.z;
        p1t.y += Sy * p1t.z;
        p2t.x += Sx * p2t.z;
        p2t.y += Sy * p2t.z;

        // Compute edge function coefficients _e0_, _e1_, and _e2_
        let mut e0 = p1t.x * p2t.y - p1t.y * p2t.x;
        let mut e1 = p2t.x * p0t.y - p2t.y * p0t.x;
        let mut e2 = p0t.x * p1t.y - p0t.y * p1t.x;

        // Fall back to double precision test at triangle edges
        if type_of(e0) == "f32" && (e0 == 0.0 || e1 == 0.0 || e2 == 0.0) {
            let p2txp1ty = p2t.x as f64 * p1t.y as f64;
            let p2typ1tx = p2t.y as f64 * p1t.x as f64;
            e0 = (p2typ1tx - p2txp1ty) as f32;
            let p0txp2ty = p0t.x as f64 * p2t.y as f64;
            let p0typ2tx = p0t.y as f64 * p2t.x as f64;
            e1 = (p0typ2tx - p0txp2ty) as f32;
            let p1txp0ty = p1t.x as f64 * p0t.y as f64;
            let p1typ0tx = p1t.y as f64 * p0t.x as f64;
            e2 = (p1typ0tx - p1txp0ty) as f32;
        }

        if (e0 < 0.0 || e1 < 0.0 || e2 < 0.0) && (e0 > 0.0 || e1 > 0.0 || e2 > 0.0) {
            return false;
        }

        let det = e0 + e1 + e2;
        if det == 0.0 {
            return false;
        }

        // Compute scaled hit distance to triangle and test against ray $t$ range
        p0t.z *= Sz;
        p1t.z *= Sz;
        p2t.z *= Sz;

        let tScaled = e0 * p0t.z + e1 * p1t.z + e2 * p2t.z;
        if det < 0.0 && (tScaled >= 0.0 || tScaled < ray.t_max * det) {
            return false;
        }
        if det > 0.0 && (tScaled <= 0.0 || tScaled > ray.t_max * det) {
            return false;
        }

        // Compute barycentric coordinates and $t$ value for triangle intersection
        let invDet = 1.0 / det;
        let b0 = e0 * invDet;
        let b1 = e1 * invDet;
        let b2 = e2 * invDet;
        let t = tScaled * invDet;
        if t < ray.t_min || t > ray.t_max {
            return false;
        }

        // Ensure that computed triangle $t$ is conservatively greater than zero

        // Compute $\delta_z$ term for triangle $t$ error bounds
        let maxZt = Vector3::new(p0t.z, p1t.z, p2t.z).abs().max_component();
        let deltaZ = gamma(3) * maxZt;

        // Compute $\delta_x$ and $\delta_y$ terms for triangle $t$ error bounds
        let maxXt = Vector3::new(p0t.x, p1t.x, p2t.x).abs().max_component();
        let maxYt = Vector3::new(p0t.y, p1t.y, p2t.y).abs().max_component();
        let deltaX = gamma(5) * (maxXt + maxZt);
        let deltaY = gamma(5) * (maxYt + maxZt);

        // Compute $\delta_e$ term for triangle $t$ error bounds
        let deltaE = 2.0 * (gamma(2) * maxXt * maxYt + deltaY * maxXt + deltaX * maxYt);

        // Compute $\delta_t$ term for triangle $t$ error bounds and check _t_
        let maxE = Vector3::new(e0, e1, e2).abs().max_component();
        let deltaT =
            3.0 * (gamma(3) * maxE * maxZt + deltaE * maxZt + deltaZ * maxE) * invDet.abs();

        if t <= deltaT {
            return false;
        };

        // Compute error bounds for triangle intersection
        let xAbsSum = ((b0 * p0.x).abs() + (b1 * p1.x).abs() + (b2 * p2.x).abs());
        let yAbsSum = ((b0 * p0.y).abs() + (b1 * p1.y).abs() + (b2 * p2.y).abs());
        let zAbsSum = ((b0 * p0.z).abs() + (b1 * p1.z).abs() + (b2 * p2.z).abs());
        let pError = gamma(7) * Vector3::new(xAbsSum, yAbsSum, zAbsSum);

        let pHit = b0 * p0 + b1 * p1 + b2 * p2;

        let mut normal = Normal::from(cross(p1 - p0, p2 - p0).normalize());
        if normal.dot(ray.d) > 0.0 {
            normal = -normal;
        }

        *interaction =
            SurfaceInteraction::new_with_error(t, pHit, pError, normal, self.material.clone());
        return true;
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
}
