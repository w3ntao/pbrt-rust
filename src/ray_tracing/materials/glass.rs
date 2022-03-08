use rand::thread_rng;
use rand_distr::{Distribution, Normal, NormalError};

use crate::fundamental::color::*;
use crate::fundamental::vector3::*;
use crate::ray_tracing::intersection::*;
use crate::ray_tracing::material::Material;
use crate::ray_tracing::ray::*;

pub struct Glass {
    index_of_refraction: f32,
}

impl Glass {
    pub fn new() -> Glass {
        Glass { index_of_refraction: 1.5 }
    }
}

fn refract(ray_in: Vector3, normal: Vector3, etai_over_etat: f32) -> Vector3 {
    let ray_in = ray_in.normalize();
    let normal = normal.normalize();
    let cost_theta = dot(-ray_in, normal);
    let ray_out_perpendicular = etai_over_etat * (ray_in + cost_theta * normal);
    let ray_out_parallel = -(1.0 - ray_out_perpendicular.length_squared()).abs().sqrt() * normal;
    return ray_out_perpendicular + ray_out_parallel;
}

impl Material for Glass {
    fn scatter(&self, scattered_ray: &mut Ray, incoming_ray: &Ray, intersection: &Intersection) -> Color {
        let refraction_ratio = {
            if intersection.entering_material {
                1.0 / self.index_of_refraction
            } else {
                // otherwise leaving the material
                self.index_of_refraction / 1.0
            }
        };

        scattered_ray.origin = intersection.ray.get_point(intersection.distance);
        scattered_ray.direction = refract(incoming_ray.direction,
                                          intersection.normal,
                                          refraction_ratio);
        return Color::new(1.0, 1.0, 1.0);
    }
}
