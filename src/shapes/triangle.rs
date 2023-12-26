use crate::pbrt::*;

#[derive(Clone)]
pub struct Triangle {
    idx: usize,
    mesh: Arc<TriangleMesh>,
}

struct TriangleIntersection {
    pub b0: f64,
    pub b1: f64,
    pub b2: f64,
    pub t: f64,
}

fn intersect_triangle(
    ray: &Ray,
    t_max: f64,
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

    let sx = -d.x / d.z;
    let sy = -d.y / d.z;
    let sz = 1.0 / d.z;

    p0t.x += sx * p0t.z;
    p0t.y += sy * p0t.z;
    p1t.x += sx * p1t.z;
    p1t.y += sy * p1t.z;
    p2t.x += sx * p2t.z;
    p2t.y += sy * p2t.z;

    let e0 = difference_of_products(p1t.x, p2t.y, p1t.y, p2t.x);
    let e1 = difference_of_products(p2t.x, p0t.y, p2t.y, p0t.x);
    let e2 = difference_of_products(p0t.x, p1t.y, p0t.y, p1t.x);

    if (e0 < 0.0 || e1 < 0.0 || e2 < 0.0) && (e0 > 0.0 || e1 > 0.0 || e2 > 0.0) {
        return None;
    }

    let det = e0 + e1 + e2;
    if det == 0.0 {
        return None;
    }

    // Compute scaled hit distance to triangle and test against ray $t$ range
    p0t.z *= sz;
    p1t.z *= sz;
    p2t.z *= sz;

    let t_scaled = e0 * p0t.z + e1 * p1t.z + e2 * p2t.z;
    if det < 0.0 && (t_scaled >= 0.0 || t_scaled < t_max * det) {
        return None;
    }

    if det > 0.0 && (t_scaled <= 0.0 || t_scaled > t_max * det) {
        return None;
    }

    // Compute barycentric coordinates and $t$ value for triangle intersection
    let inv_det = 1.0 / det;
    let b0 = e0 * inv_det;
    let b1 = e1 * inv_det;
    let b2 = e2 * inv_det;
    let t = t_scaled * inv_det;

    // Ensure that computed triangle $t$ is conservatively greater than zero
    // Compute $\delta_z$ term for triangle $t$ error bounds
    let max_zt = Vector3f::new(p0t.z, p1t.z, p2t.z)
        .abs()
        .max_component_value();

    let delta_z = gamma(3) * max_zt;

    // Compute $\delta_x$ and $\delta_y$ terms for triangle $t$ error bounds
    let max_xt = Vector3f::new(p0t.x, p1t.x, p2t.x)
        .abs()
        .max_component_value();

    let max_yt = Vector3f::new(p0t.y, p1t.y, p2t.y)
        .abs()
        .max_component_value();

    let delta_x = gamma(5) * (max_xt + max_zt);
    let delta_y = gamma(5) * (max_yt + max_zt);

    // Compute $\delta_e$ term for triangle $t$ error bounds
    let delta_e = 2.0 * (gamma(2) * max_xt * max_yt + delta_y * max_xt + delta_x * max_yt);

    // Compute $\delta_t$ term for triangle $t$ error bounds and check _t_
    let max_e = Vector3f::new(e0, e1, e2).abs().max_component_value();

    let delta_t =
        3.0 * (gamma(3) * max_e * max_zt + delta_e * max_zt + delta_z * max_e) * inv_det.abs();

    if t <= delta_t {
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

        return (self.mesh.p[v0], self.mesh.p[v1], self.mesh.p[v2]);
    }

    fn solid_angle(&self, p: Point3f) -> f64 {
        let (p0, p1, p2) = self.get_points();

        return spherical_triangle_area(
            (p0 - p).normalize(),
            (p1 - p).normalize(),
            (p2 - p).normalize(),
        );
    }

    fn interaction_from_intersection(
        &self,
        ti: &TriangleIntersection,
        wo: Vector3f,
    ) -> SurfaceInteraction {
        // Compute triangle partial derivatives
        // Compute deltas and matrix determinant for triangle partial derivatives
        // Get triangle texture coordinates in _uv_ array

        let v0 = self.mesh.indices[self.idx + 0];
        let v1 = self.mesh.indices[self.idx + 1];
        let v2 = self.mesh.indices[self.idx + 2];

        let p0 = self.mesh.p[v0];
        let p1 = self.mesh.p[v1];
        let p2 = self.mesh.p[v2];

        let uv = if self.mesh.uv.len() > 0 {
            let _uv = &self.mesh.uv;
            [_uv[v0], _uv[v1], _uv[v2]]
        } else {
            [
                Point2f::new(0.0, 0.0),
                Point2f::new(1.0, 0.0),
                Point2f::new(1.0, 1.0),
            ]
        };

        let duv02 = uv[0] - uv[2];
        let duv12 = uv[1] - uv[2];

        let dp02 = p0 - p2;
        let dp12 = p1 - p2;

        let (dpdu, dpdv) = {
            let mut dpdu = Vector3f::nan();
            let mut dpdv = Vector3f::nan();

            let determinant = difference_of_products(duv02.x, duv12.y, duv02.y, duv12.x);
            let degenerate_uv = determinant.abs() < 1e-9;
            if !degenerate_uv {
                // Compute triangle $\dpdu$ and $\dpdv$ via matrix inversion
                let inv_det = 1.0 / determinant;
                dpdu = difference_of_products_vec3(duv12[1], dp02, duv02[1], dp12) * inv_det;
                dpdv = difference_of_products_vec3(duv02[0], dp12, duv12[0], dp02) * inv_det;
            }
            if degenerate_uv || dpdu.cross(dpdv).length_squared() == 0.0 {
                let _ng = (p2 - p0).cross(p1 - p0);
                let ng = if _ng.length_squared() == 0.0 {
                    (p2 - p0).cross(p1 - p0)
                } else {
                    _ng
                };

                (dpdu, dpdv) = ng.normalize().coordinate_system();
            }

            (dpdu, dpdv)
        };

        // Interpolate $(u,v)$ parametric coordinates and hit point
        let p_hit = ti.b0 * p0 + ti.b1 * p1 + ti.b2 * p2;
        let uv_hit = ti.b0 * uv[0] + ti.b1 * uv[1] + ti.b2 * uv[2];

        // Compute error bounds _pError_ for triangle intersection
        let p_abs_sum = (ti.b0 * p0).abs() + (ti.b1 * p1).abs() + (ti.b2 * p2).abs();
        let p_error = gamma(7) * Vector3f::from(p_abs_sum);

        let mut isect = SurfaceInteraction::new(
            Point3fi::from_value_and_error(p_hit, p_error),
            uv_hit,
            wo,
            dpdu,
            dpdv,
            Normal3f::nan(),
            Normal3f::nan(),
        );

        (isect.interaction.n, isect.shading.n) = {
            let n = Normal3f::from(dp02.cross(dp12).normalize());
            if self.mesh.reverse_orientation {
                (-n, -n)
            } else {
                (n, n)
            }
        };

        return isect;
    }
}

impl Shape for Triangle {
    fn intersect(&self, ray: &Ray, t_max: f64) -> Option<ShapeIntersection> {
        let (p0, p1, p2) = self.get_points();

        let triangle_intersection = match intersect_triangle(ray, t_max, p0, p1, p2) {
            None => {
                return None;
            }
            Some(_ti) => _ti,
        };

        return Some(ShapeIntersection {
            t_hit: triangle_intersection.t,
            surface_interaction: self.interaction_from_intersection(&triangle_intersection, -ray.d),
        });
    }

    fn fast_intersect(&self, ray: &Ray, t_max: f64) -> bool {
        let (p0, p1, p2) = self.get_points();
        return intersect_triangle(ray, t_max, p0, p1, p2).is_some();
    }

    fn bounds(&self) -> Bounds3f {
        let (p0, p1, p2) = self.get_points();
        return Bounds3f::from_multiple_points(&[p0, p1, p2]);
    }

    fn area(&self) -> f64 {
        let (p0, p1, p2) = self.get_points();
        return (p1 - p0).cross(p2 - p0).length() * 0.5;
    }

    fn sample(&self, u: Point2f) -> Option<ShapeSample> {
        // Get triangle vertices in _p0_, _p1_, and _p2_

        let v = [
            self.mesh.indices[self.idx + 0],
            self.mesh.indices[self.idx + 1],
            self.mesh.indices[self.idx + 2],
        ];

        let p0 = self.mesh.p[v[0]];
        let p1 = self.mesh.p[v[1]];
        let p2 = self.mesh.p[v[2]];

        // Sample point on triangle uniformly by area
        let b = sample_uniform_triangle(u);
        let p = b[0] * p0 + b[1] * p1 + b[2] * p2;

        // Compute surface normal for sampled point on triangle
        let n = {
            let n = Normal3f::from((p1 - p0).cross(p2 - p0)).normalize();
            if self.mesh.n.len() > 0 {
                let ns = b[0] * self.mesh.n[v[0]]
                    + b[1] * self.mesh.n[v[1]]
                    + (1.0 - b[0] - b[1]) * self.mesh.n[v[2]];

                n.face_forward(ns.into())
            } else if self.mesh.reverse_orientation {
                -n
            } else {
                n
            }
        };

        // Compute $(u,v)$ for sampled point on triangle
        // Get triangle texture coordinates in _uv_ array
        let uv = if self.mesh.uv.len() > 0 {
            [self.mesh.uv[v[0]], self.mesh.uv[v[1]], self.mesh.uv[v[2]]]
        } else {
            [
                Point2f::new(0.0, 0.0),
                Point2f::new(1.0, 0.0),
                Point2f::new(1.0, 1.0),
            ]
        };

        let uv_sample = b[0] * uv[0] + b[1] * uv[1] + b[2] * uv[2];
        // Compute error bounds _pError_ for sampled point on triangle

        let p_abs_sum = (b[0] * p0).abs() + (b[1] * p1).abs() + ((1.0 - b[0] - b[1]) * p2).abs();
        let p_error = gamma(6) * p_abs_sum;

        let shape_sample = ShapeSample {
            interaction: Interaction {
                pi: Point3fi::from_value_and_error(p, p_error.into()),
                n,
                wo: Vector3f::nan(),
                uv: uv_sample,
            },
            pdf: 1.0 / self.area(),
        };

        return Some(shape_sample);
    }

    fn sample_with_context(&self, ctx: &ShapeSampleContext, u: Point2f) -> Option<ShapeSample> {
        let v = [
            self.mesh.indices[self.idx + 0],
            self.mesh.indices[self.idx + 1],
            self.mesh.indices[self.idx + 2],
        ];
        let (p0, p1, p2) = (self.mesh.p[v[0]], self.mesh.p[v[1]], self.mesh.p[v[2]]);

        // Use uniform area sampling for numerically unstable cases
        let solid_angle = self.solid_angle(ctx.pi.into());
        if solid_angle < MIN_SPHERICAL_SAMPLE_AREA || solid_angle > MAX_SPHERICAL_SAMPLE_AREA {
            // Sample shape by area and compute incident direction _wi_
            let mut ss = self.sample(u).unwrap();
            let _wi = Point3f::from(ss.interaction.pi) - Point3f::from(ctx.pi);

            if _wi.length_squared() == 0.0 {
                return None;
            }
            let wi = _wi.normalize();

            ss.pdf /= ss.interaction.n.dot(-wi).abs()
                / (Point3f::from(ctx.pi) - Point3f::from(ss.interaction.pi)).length_squared();
            if ss.pdf.is_infinite() {
                return None;
            }

            return Some(ss);
        }

        // Sample spherical triangle from reference point
        // Apply warp product sampling for cosine factor at reference point

        let mut pdf = 1.0;
        let mut u = u;
        if ctx.ns.is_valid() {
            // This part is slightly different with PBRT-v4
            // Compute $\cos\theta$-based weights _w_ at sample domain corners

            let rp = Point3f::from(ctx.pi);
            let wi = [
                (p0 - rp).normalize(),
                (p1 - rp).normalize(),
                (p2 - rp).normalize(),
            ];

            let w = [
                ctx.ns.dot(wi[1]).abs().max(0.01),
                ctx.ns.dot(wi[1]).abs().max(0.01),
                ctx.ns.dot(wi[0]).abs().max(0.01),
                ctx.ns.dot(wi[2]).abs().max(0.01),
            ];

            u = sample_bilinear(u, &w);
            pdf = bilinear_pdf(u, &w);
        }

        let (b, tri_pdf) = sample_spherical_triangle(&[p0, p1, p2], ctx.pi.into(), u);
        if tri_pdf == 0.0 {
            return None;
        }
        pdf *= tri_pdf;

        // Compute error bounds _pError_ for sampled point on triangle
        let p_abs_sum = (b[0] * p0).abs() + (b[1] * p1).abs() + ((1.0 - b[0] - b[1]) * p2).abs();
        let p_error = Vector3f::from(gamma(6) * p_abs_sum);

        // Return _ShapeSample_ for solid angle sampled point on triangle
        let p = b[0] * p0 + b[1] * p1 + b[2] * p2;

        // Compute surface normal for sampled point on triangle

        let n = {
            let n = Normal3f::from((p1 - p0).cross(p2 - p0)).normalize();

            if self.mesh.n.len() > 0 {
                let ns = b[0] * self.mesh.n[v[0]]
                    + b[1] * self.mesh.n[v[1]]
                    + (1.0 - b[0] - b[1]) * self.mesh.n[v[2]];

                n.face_forward(ns.into())
            } else if self.mesh.reverse_orientation {
                n * -1.0
            } else {
                n
            }
        };

        // Compute $(u,v)$ for sampled point on triangle
        // Get triangle texture coordinates in _uv_ array

        let uv = if self.mesh.uv.len() > 0 {
            [self.mesh.uv[v[0]], self.mesh.uv[v[1]], self.mesh.uv[v[2]]]
        } else {
            [
                Point2f::new(0.0, 0.0),
                Point2f::new(1.0, 0.0),
                Point2f::new(1.0, 1.0),
            ]
        };

        let uv_sample = b[0] * uv[0] + b[1] * uv[1] + b[2] * uv[2];
        let shape_sample = ShapeSample {
            interaction: Interaction {
                pi: Point3fi::from_value_and_error(p, p_error),
                n,
                wo: Vector3::nan(),
                uv: uv_sample,
            },
            pdf,
        };

        return Some(shape_sample);
    }
}
