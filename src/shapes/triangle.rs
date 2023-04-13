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
}

impl Triangle {
    pub fn new(_index: usize, _mesh: Arc<TriangleMesh>) -> Self {
        return Self {
            mesh_index: _index,
            mesh_root: _mesh,
        };
    }
}

impl Shape for Triangle {
    fn intersect(&self, ray: &Ray, t_hit: &mut f32) -> Option<SurfaceInteraction> {
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

        let s_x = -d.x / d.z;
        let s_y = -d.y / d.z;
        let s_z = 1.0 / d.z;

        p0t.x += s_x * p0t.z;
        p0t.y += s_y * p0t.z;
        p1t.x += s_x * p1t.z;
        p1t.y += s_y * p1t.z;
        p2t.x += s_x * p2t.z;
        p2t.y += s_y * p2t.z;

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
            return None;
        }

        let det = e0 + e1 + e2;
        if det == 0.0 {
            return None;
        }

        // Compute scaled hit distance to triangle and test against ray $t$ range
        p0t.z *= s_z;
        p1t.z *= s_z;
        p2t.z *= s_z;

        let t_scaled = e0 * p0t.z + e1 * p1t.z + e2 * p2t.z;
        if det < 0.0 && (t_scaled >= 0.0 || t_scaled < ray.t_max * det) {
            return None;
        }
        if det > 0.0 && (t_scaled <= 0.0 || t_scaled > ray.t_max * det) {
            return None;
        }

        // Compute barycentric coordinates and $t$ value for triangle intersection
        let inv_det = 1.0 / det;
        let b0 = e0 * inv_det;
        let b1 = e1 * inv_det;
        let b2 = e2 * inv_det;
        let t = t_scaled * inv_det;
        if t < 0.0 || t > ray.t_max {
            return None;
        }

        // Ensure that computed triangle $t$ is conservatively greater than zero

        // Compute $\delta_z$ term for triangle $t$ error bounds
        let max_zt = Vector3::new(p0t.z, p1t.z, p2t.z).abs().max_component();
        let delta_z = gamma(3) * max_zt;

        // Compute $\delta_x$ and $\delta_y$ terms for triangle $t$ error bounds
        let max_xt = Vector3::new(p0t.x, p1t.x, p2t.x).abs().max_component();
        let max_yt = Vector3::new(p0t.y, p1t.y, p2t.y).abs().max_component();
        let delta_x = gamma(5) * (max_xt + max_zt);
        let delta_y = gamma(5) * (max_yt + max_zt);

        // Compute $\delta_e$ term for triangle $t$ error bounds
        let delta_e = 2.0 * (gamma(2) * max_xt * max_yt + delta_y * max_xt + delta_x * max_yt);

        // Compute $\delta_t$ term for triangle $t$ error bounds and check _t_
        let max_e = Vector3::new(e0, e1, e2).abs().max_component();
        let delta_t =
            3.0 * (gamma(3) * max_e * max_zt + delta_e * max_zt + delta_z * max_e) * inv_det.abs();

        if t <= delta_t {
            return None;
        };

        // Compute error bounds for triangle intersection
        let x_abs_sum = (b0 * p0.x).abs() + (b1 * p1.x).abs() + (b2 * p2.x).abs();
        let y_abs_sum = (b0 * p0.y).abs() + (b1 * p1.y).abs() + (b2 * p2.y).abs();
        let z_abs_sum = (b0 * p0.z).abs() + (b1 * p1.z).abs() + (b2 * p2.z).abs();
        let p_error = gamma(7) * Vector3::new(x_abs_sum, y_abs_sum, z_abs_sum);

        let p_hit = b0 * p0 + b1 * p1 + b2 * p2;

        let mut normal = Normal::from(cross(p1 - p0, p2 - p0).normalize());
        if normal.dot(ray.d) > 0.0 {
            normal = -normal;
        }

        *t_hit = t;

        let mut si = SurfaceInteraction::default();
        si.p = p_hit;
        si.n = normal;
        si.p_error = p_error;

        return Some(si);
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
}
