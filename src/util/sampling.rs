use crate::pbrt::*;

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

pub fn cosine_hemisphere_pdf(cos_theta: Float) -> Float {
    return cos_theta * INV_PI;
}
