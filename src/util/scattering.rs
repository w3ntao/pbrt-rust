use crate::pbrt::*;

pub fn reflect(wo: Vector3f, n: Vector3f) -> Vector3f {
    return -wo + 2.0 * wo.dot(n) * n;
}

pub fn refract(wi: Vector3f, n: Normal3f, eta: f64) -> (bool, f64, Vector3f) {
    let cos_theta_i = n.dot(wi);
    // Potentially flip interface orientation for Snell's law
    let (eta, cos_theta_i, n) = if cos_theta_i < 0.0 {
        (1.0 / eta, -cos_theta_i, -n)
    } else {
        (eta, cos_theta_i, n)
    };

    // Compute $\cos\,\theta_\roman{t}$ using Snell's law
    let sin2theta_i = (1.0 - sqr(cos_theta_i)).max(0.0);
    let sin2theta_t = sin2theta_i / sqr(eta);
    // Handle total internal reflection case
    if sin2theta_t >= 1.0 {
        return (false, f64::NAN, Vector3f::nan());
    }

    let cos_theta_t = (1.0 - sin2theta_t).sqrt();
    let wt = -wi / eta + (cos_theta_i / eta - cos_theta_t) * Vector3f::from(n);

    // eta: Provide relative IOR along ray to caller
    return (true, eta, wt);
}

pub fn fr_dielectric(cos_theta_i: f64, _eta: f64) -> f64 {
    let _cos_theta_i = cos_theta_i.clamp(-1.0, 1.0);
    // Potentially flip interface orientation for Fresnel equations
    let (cos_theta_i, eta) = if _cos_theta_i < 0.0 {
        (-_cos_theta_i, 1.0 / _eta)
    } else {
        (_cos_theta_i, _eta)
    };

    // Compute $\cos\,\theta_\roman{t}$ for Fresnel equations using Snell's law

    let sin2_theta_i = 1.0 - sqr(cos_theta_i);
    let sin2_theta_t = sin2_theta_i / sqr(eta);
    if sin2_theta_t >= 1.0 {
        return 1.0;
    }

    let cos_theta_t = safe_sqrt(1.0 - sin2_theta_t);
    let r_parallel = (eta * cos_theta_i - cos_theta_t) / (eta * cos_theta_i + cos_theta_t);
    let r_perpendicular = (cos_theta_i - eta * cos_theta_t) / (cos_theta_i + eta * cos_theta_t);

    return (sqr(r_parallel) + sqr(r_perpendicular)) / 2.0;
}

pub fn henyey_greenstein(cos_theta: f64, g: f64) -> f64 {
    // The Henyey-Greenstein phase function isn't suitable for |g| \approx
    // 1 so we clamp it before it becomes numerically instable. (It's an
    // analogous situation to BSDFs: if the BSDF is perfectly specular, one
    // should use one based on a Dirac delta distribution rather than a
    // very smooth microfacet distribution...)
    let g = g.clamp(-0.99, 0.99);
    let denom = 1.0 + sqr(g) + 2.0 * g * cos_theta;
    return INV_4PI * (1.0 - sqr(g)) / (denom * safe_sqrt(denom));
}

#[derive(Clone, Copy)]
pub struct TrowbridgeReitzDistribution {
    alpha_x: f64,
    alpha_y: f64,
}

impl TrowbridgeReitzDistribution {
    pub fn new(alpha_x: f64, alpha_y: f64) -> Self {
        let mut distribution = TrowbridgeReitzDistribution { alpha_x, alpha_y };

        if !distribution.effectively_smooth() {
            // If one direction has some roughness, then the other can't
            // have zero (or very low) roughness; the computation of |e| in
            // D() blows up in that case.

            distribution.alpha_x = distribution.alpha_x.max(1e-4);
            distribution.alpha_y = distribution.alpha_y.max(1e-4);
        }

        return distribution;
    }

    pub fn effectively_smooth(&self) -> bool {
        return self.alpha_x.max(self.alpha_y) < 1e-3;
    }

    pub fn lambda(&self, w: Vector3f) -> f64 {
        let tan2theta = w.tan2_theta();
        if tan2theta.is_infinite() {
            return 0.0;
        }

        let alpha2 = sqr(w.cos_phi() * self.alpha_x) + sqr(w.sin_phi() * self.alpha_y);
        return ((1.0 + alpha2 * tan2theta).sqrt() - 1.0) / 2.0;
    }

    pub fn g(&self, wo: Vector3f, wi: Vector3f) -> f64 {
        return 1.0 / (1.0 + self.lambda(wo) + self.lambda(wi));
    }

    pub fn g1(&self, w: Vector3f) -> f64 {
        return 1.0 / (1.0 + self.lambda(w));
    }

    pub fn d1(&self, wm: Vector3f) -> f64 {
        let tan2_theta = wm.tan2_theta();
        if tan2_theta.is_infinite() {
            return 0.0;
        }

        let cos4_theta = sqr(wm.cos2_theta());
        if cos4_theta < 1e-16 {
            return 0.0;
        }

        let e = tan2_theta * (sqr(wm.cos_phi() / self.alpha_x) + sqr(wm.sin_phi() / self.alpha_y));

        return 1.0 / (PI * self.alpha_x * self.alpha_y * cos4_theta * sqr(1.0 + e));
    }

    pub fn d2(&self, w: Vector3f, wm: Vector3f) -> f64 {
        return self.g1(w) / w.abs_cos_theta() * self.d1(wm) * w.abs_dot(wm);
    }

    pub fn pdf(&self, w: Vector3f, wm: Vector3f) -> f64 {
        return self.d2(w, wm);
    }

    pub fn sample_wm(&self, w: Vector3f, u: Point2f) -> Vector3f {
        // Transform _w_ to hemispherical configuration
        let mut wh = Vector3f::new(self.alpha_x * w.x, self.alpha_y * w.y, w.z).normalize();
        if wh.z < 0.0 {
            wh = -wh;
        }

        // Find orthonormal basis for visible normal sampling
        let T1 = if wh.z < 0.99999 {
            Vector3f::new(0.0, 0.0, 1.0).cross(wh).normalize()
        } else {
            Vector3f::new(1.0, 0.0, 0.0)
        };
        let T2 = wh.cross(T1);

        // Generate uniformly distributed points on the unit disk
        let mut p = sample_uniform_disk_polar(u);

        // Warp hemispherical projection for visible normal sampling
        let h = (1.0 - sqr(p.x)).sqrt();
        p.y = lerp((1.0 + wh.z) / 2.0, h, p.y);

        // Reproject to hemisphere and transform normal to ellipsoid configuration
        let pz = (1.0 - Vector2f::from(p).length_squared()).max(0.0).sqrt();
        let nh = p.x * T1 + p.y * T2 + pz * wh;

        return Vector3f::new(self.alpha_x * nh.x, self.alpha_y * nh.y, nh.z.max(1e-6)).normalize();
    }
}
