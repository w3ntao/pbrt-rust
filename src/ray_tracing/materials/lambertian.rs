use rand_distr::{Distribution, Normal, NormalError};
use rand::thread_rng;

use crate::fundamental::rgb_color::*;
use crate::fundamental::vector::*;
use crate::ray_tracing::ray::*;
use crate::ray_tracing::intersection::*;
use crate::ray_tracing::material::Material;

pub struct Lambertian {
    pub(crate) albedo: RGBColor,
}

fn random_in_unit_sphere() -> Vector {
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
        return Vector::new(x, y, z);
    }
}

fn random_vector_in_hemisphere(normal: &Vector) -> Vector {
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
    fn scatter(&self, attenuation: &mut RGBColor, scattered_ray: &mut Ray, incoming_ray: &Ray, intersect: &Intersection) -> bool {
        let scattered_direction = random_vector_in_hemisphere(&intersect.normal);

        scattered_ray.origin = intersect.ray.get_point(intersect.distance);
        scattered_ray.direction = scattered_direction.normalize();
        *attenuation = self.albedo;
        return true;
    }
}
