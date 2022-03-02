use rand::thread_rng;
use rand_distr::{Distribution, Normal, NormalError};

use crate::fundamental::color::*;
use crate::fundamental::vector3::*;
use crate::ray_tracing::intersection::*;
use crate::ray_tracing::material::Material;
use crate::ray_tracing::ray::*;

pub struct Lambertian {
    pub albedo: Color,
}

fn random_in_unit_sphere() -> Vector3 {
    // TODO: this is inefficient
    let mut rng = thread_rng();
    let normal = Normal::new(-1.0, 1.0).unwrap();

    loop {
        let x = normal.sample(&mut rng);
        let y = normal.sample(&mut rng);
        let z = normal.sample(&mut rng);

        let acc = x * x + y * y + z * z;
        if acc > 1.0 || acc < 0.0001 {
            continue;
        }
        return Vector3::new(x, y, z);
    }
}

fn random_vector_in_hemisphere(normal: &Vector3) -> Vector3 {
    let random_vec = random_in_unit_sphere();

    return {
        if dot(&random_vec, normal) < 0.0 {
            -random_vec
        } else {
            random_vec
        }
    };
}

impl Material for Lambertian {
    fn scatter(&self, scattered_ray: &mut Ray, _: &Ray, intersection: &Intersection) -> Color {
        let scattered_direction = random_vector_in_hemisphere(&intersection.normal);

        scattered_ray.origin = intersection.ray.get_point(intersection.distance) + 0.001 * intersection.normal;
        scattered_ray.direction = scattered_direction;
        return self.albedo;
    }
}
