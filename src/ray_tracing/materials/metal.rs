use rand::thread_rng;
use rand_distr::{Distribution, Normal, NormalError};

use crate::fundamental::color::*;
use crate::fundamental::vector3::*;
use crate::ray_tracing::intersection::*;
use crate::ray_tracing::material::Material;
use crate::ray_tracing::ray::*;

pub struct Metal {
    pub albedo: Color,
    pub fuzz: f32,
}

impl Metal {
    pub fn new(_albedo: &Color, _fuzz: f32) -> Metal {
        Metal {
            albedo: _albedo.clone(),
            fuzz: _fuzz,
        }
    }
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

fn reflect(vec_in: &Vector3, normal: &Vector3) -> Vector3 {
    return vec_in.clone() - 2.0 * dot(vec_in, normal) * normal.clone();
}

impl Material for Metal {
    fn scatter(&self, scattered_ray: &mut Ray, incoming_ray: &Ray, intersection: &Intersection) -> Color {
        let reflected = reflect(&incoming_ray.direction.normalize(), &intersection.normal);
        scattered_ray.origin = intersection.ray.get_point(intersection.distance) + 0.001 * intersection.normal;
        scattered_ray.direction = reflected + self.fuzz * random_in_unit_sphere();

        if dot(&scattered_ray.direction, &intersection.normal) <= 0.0 {
            return Color::black();
        }

        return self.albedo;
    }
}
