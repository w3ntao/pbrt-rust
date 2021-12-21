use rand_distr::{Distribution, Normal, NormalError};
use rand::thread_rng;

use crate::fundamental::rgb_color::*;
use crate::fundamental::vector::*;
use crate::ray_tracing::ray::*;
use crate::ray_tracing::intersection::*;
use crate::ray_tracing::material::Material;

pub struct Metal {
    pub albedo: RGBColor,
    pub fuzz: f32,
}

impl Metal {
    pub fn new(_albedo: &RGBColor, _fuzz: f32) -> Metal {
        Metal {
            albedo: _albedo.clone(),
            fuzz: _fuzz,
        }
    }
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

fn reflect(vec_in: &Vector, normal: &Vector) -> Vector {
    return vec_in.clone() - 2.0 * dot(vec_in, normal) * normal.clone();
}

impl Material for Metal {
    fn scatter(&self, attenuation: &mut RGBColor, scattered_ray: &mut Ray, incoming_ray: &Ray, intersect: &Intersection) -> bool {
        let reflected = reflect(&incoming_ray.direction.normalize(), &intersect.normal);
        scattered_ray.origin = intersect.ray.get_point(intersect.distance) + 0.001 * intersect.normal;
        scattered_ray.direction = reflected + self.fuzz * random_in_unit_sphere();
        *attenuation = self.albedo;

        return dot(&scattered_ray.direction, &intersect.normal) > 0.0;
    }
}
