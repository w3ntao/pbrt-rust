use crate::pbrt::*;

/*
PBRT_CPU_GPU inline pstd::array<Float, 3> SampleUniformTriangle(Point2f u) {
    Float b0, b1;
    if (u[0] < u[1]) {
        b0 = u[0] / 2;
        b1 = u[1] - b0;
    } else {
        b1 = u[1] / 2;
        b0 = u[0] - b1;
    }
    return {b0, b1, 1 - b0 - b1};
}
*/

pub fn sample_uniform_triangle(u: Point2f) -> [f64; 3] {
    let (b0, b1) = {
        if u[0] < u[1] {
            let b0 = u[0] / 2.0;
            let b1 = u[1] - b0;
            (b0, b1)
        } else {
            let b1 = u[1] / 2.0;
            let b0 = u[0] - b1;
            (b0, b1)
        }
    };

    return [b0, b1, 1.0 - b0 - b1];
}

pub fn sample_uniform_disk_concentric(u: Point2f) -> Point2f {
    // Map _u_ to $[-1,1]^2$ and handle degeneracy at the origin

    let u_offset = 2.0 * u - Vector2f::new(1.0, 1.0);
    if u_offset.x == 0.0 && u_offset.y == 0.0 {
        return Point2f::new(0.0, 0.0);
    }

    // Apply concentric mapping to point
    let (r, theta) = if u_offset.x.abs() > u_offset.y.abs() {
        (u_offset.x, PI_OVER_4 * (u_offset.y / u_offset.x))
    } else {
        (
            u_offset.y,
            PI_OVER_2 - PI_OVER_4 * (u_offset.x / u_offset.y),
        )
    };

    return r * Point2f::new(theta.cos(), theta.sin());
}

pub fn sample_cosine_hemisphere(u: Point2f) -> Vector3f {
    let d = sample_uniform_disk_concentric(u);
    let z = (1.0 - d.x * d.x - d.y * d.y).sqrt();

    return Vector3f::new(d.x, d.y, z);
}

pub const fn cosine_hemisphere_pdf(cos_theta: f64) -> f64 {
    return cos_theta * INV_PI;
}

pub fn sample_visible_wavelengths(u: f64) -> f64 {
    return 538.0 - 138.888889 * (0.85691062 - 1.82750197 * u).atanh();
}

pub fn visible_wavelengths_pdf(lambda: f64) -> f64 {
    if lambda < 360.0 || lambda > 830.0 {
        return 0.0;
    }

    return 0.0039398042 / sqr((0.0072 * (lambda - 538.0)).cosh());
}

pub fn sample_linear(u: f64, a: f64, b: f64) -> f64 {
    if u == 0.0 && a == 0.0 {
        return 0.0;
    }

    let x = u * (a + b) / (a + lerp(u, a * a, b * b).sqrt());
    return x.min(ONE_MINUS_EPSILON);
}

pub fn sample_bilinear(u: Point2f, w: &[f64; 4]) -> Point2f {
    let y = sample_linear(u[1], w[0] + w[1], w[2] + w[3]);
    let x = sample_linear(u[0], lerp(y, w[0], w[2]), lerp(y, w[1], w[3]));

    return Point2f::new(x, y);
}

pub fn bilinear_pdf(p: Point2f, w: &[f64; 4]) -> f64 {
    let sum = w[0] + w[1] + w[2] + w[3];

    if sum == 0.0 {
        return 1.0;
    }

    return 4.0
        * ((1.0 - p[0]) * (1.0 - p[1]) * w[0]
            + p[0] * (1.0 - p[1]) * w[1]
            + (1.0 - p[0]) * p[1] * w[2]
            + p[0] * p[1] * w[3])
        / sum;
}

pub fn sample_spherical_triangle(v: &[Point3f; 3], p: Point3f, u: Point2f) -> ([f64; 3], f64) {
    // Compute vectors _a_, _b_, and _c_ to spherical triangle vertices

    let a = (v[0] - p).normalize();
    let b = (v[1] - p).normalize();
    let c = (v[2] - p).normalize();

    // Compute normalized cross products of all direction pairs
    let n_ab = a.cross(b);
    let n_bc = b.cross(c);
    let n_ca = c.cross(a);

    if n_ab.length_squared() == 0.0 || n_bc.length_squared() == 0.0 || n_ca.length_squared() == 0.0
    {
        return ([f64::NAN, f64::NAN, f64::NAN], 0.0);
    }

    let n_ab = n_ab.normalize();
    let n_bc = n_bc.normalize();
    let n_ca = n_ca.normalize();

    // Find angles $\alpha$, $\beta$, and $\gamma$ at spherical triangle vertices
    let alpha = n_ab.angle_between(-n_ca);
    let beta = n_bc.angle_between(-n_ab);
    let gamma = n_ca.angle_between(-n_bc);

    // Uniformly sample triangle area $A$ to compute $A'$
    let A_pi = alpha + beta + gamma;
    let Ap_pi = lerp(u[0], PI, A_pi);

    let pdf = {
        let A = A_pi - PI;
        if A <= 0.0 {
            0.0
        } else {
            1.0 / A
        }
    };

    // Find $\cos\beta'$ for point along _b_ for sampled area
    let cos_alpha = alpha.cos();
    let sin_alpha = alpha.sin();
    let sin_phi = Ap_pi.sin() * cos_alpha - Ap_pi.cos() * sin_alpha;
    let cos_phi = Ap_pi.cos() * cos_alpha + Ap_pi.sin() * sin_alpha;

    let k1 = cos_phi + cos_alpha;
    let k2 = sin_phi - sin_alpha * a.dot(b);

    let cos_bp = (k2 + difference_of_products(k2, cos_phi, k1, sin_phi) * cos_alpha)
        / ((sum_of_products(k2, sin_phi, k1, cos_phi)) * sin_alpha);

    // Happens if the triangle basically covers the entire hemisphere.
    // We currently depend on calling code to detect this case, which
    // is sort of ugly/unfortunate.
    let cos_bp = cos_bp.clamp(-1.0, 1.0);

    // Sample $c'$ along the arc between $b'$ and $a$
    let sin_bp = safe_sqrt(1.0 - sqr(cos_bp));
    let cp = cos_bp * a + sin_bp * gram_schmidt(c, a).normalize();

    // Compute sampled spherical triangle direction and return barycentrics

    let cos_theta = 1.0 - u[1] * (1.0 - cp.dot(b));
    let sin_theta = safe_sqrt(1.0 - sqr(cos_theta));

    let w = cos_theta * b + sin_theta * gram_schmidt(cp, b).normalize();

    // Find barycentric coordinates for sampled direction _w_
    let e1 = v[1] - v[0];
    let e2 = v[2] - v[0];
    let s1 = w.cross(e2);
    let divisor = s1.dot(e1);

    if divisor == 0.0 {
        // This happens with triangles that cover (nearly) the whole
        // hemisphere.
        return ([1.0 / 3.0; 3], pdf);
    }

    let inv_divisor = 1.0 / divisor;
    let s = p - v[0];
    let b1 = s.dot(s1) * inv_divisor;
    let b2 = w.dot(s.cross(e1)) * inv_divisor;

    // Return clamped barycentrics for sampled direction
    let (b1, b2) = {
        let _b1 = b1.clamp(0.0, 1.0);
        let _b2 = b2.clamp(0.0, 1.0);

        let sum = _b1 + _b2;
        if sum > 1.0 {
            (_b1 / sum, _b2 / sum)
        } else {
            (_b1, _b2)
        }
    };

    return ([1.0 - b1 - b2, b1, b2], pdf);
}
