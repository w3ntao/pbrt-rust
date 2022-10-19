use crate::core::interfaces::*;
use crate::core::random::random_f32;
use crate::fundamental::orthonormal_basis::OrthonormalBasis;
use std::sync::Arc;

pub struct Lambertian {
    albedo: Arc<dyn Texture>,
}

impl Lambertian {
    pub fn new(texture: Arc<dyn Texture>) -> Lambertian {
        Lambertian { albedo: texture }
    }
}

fn random_cosine_direction() -> Vector3 {
    let sin2_theta = random_f32(0.0, 1.0);
    let cos2_theta = 1.0 - sin2_theta;

    let phi = random_f32(0.0, 2.0 * PI);
    let sin_phi = phi.sin();
    let cos_phi = phi.cos();

    let sin_theta = sin2_theta.sqrt();

    return Vector3::new(sin_phi * sin_theta, cos_phi * sin_theta, cos2_theta.sqrt());
}

impl Material for Lambertian {
    fn scatter(&self, _: Ray, intersection: &Intersection) -> (bool, Ray, Color) {
        let uvw = OrthonormalBasis::build_from_w(intersection.normal);
        let random_direction = uvw.local(random_cosine_direction());

        let scattered_ray = Ray::new(intersection.hit_point, random_direction.normalize());

        return (
            true,
            scattered_ray,
            self.albedo
                .get_color(intersection.u, intersection.v, intersection.hit_point),
        );
    }

    fn scattering_pdf(&self, _: Vector3, normal: Vector3, scattered_direction: Vector3) -> f32 {
        let val_cosine = cosine(normal, scattered_direction);

        return if val_cosine <= 0.0 {
            0.0
        } else {
            val_cosine / PI
        };
    }
}
