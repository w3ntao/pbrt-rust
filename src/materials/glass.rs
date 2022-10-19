use crate::core::color::*;
use crate::core::intersection::*;
use crate::core::material::Material;
use crate::core::ray::*;
use crate::core::vector3::*;
use crate::fundamental::random::random_f32;
use rand_distr::num_traits::Pow;

pub struct Glass {
    index_of_refraction: f32,
}

impl Glass {
    pub fn new(_index_of_refraction: f32) -> Glass {
        Glass {
            index_of_refraction: _index_of_refraction,
        }
    }
}

fn refract(uv: Vector3, n: Vector3, etai_over_etat: f32) -> Vector3 {
    let cos_theta = dot(-uv, n).min(1.0);
    let r_out_perp = etai_over_etat * (uv + cos_theta * n);
    let r_out_parallel = -(1.0 - r_out_perp.length_squared()).abs().sqrt() * n;
    return r_out_perp + r_out_parallel;
}

fn reflectance(cosine: f32, index_of_refraction: f32) -> f32 {
    // Use Schlick's approximation for reflectance. Taken from
    // https://raytracing.github.io/books/RayTracingInOneWeekend.html#dielectrics/schlickapproximation
    let r0 = (1.0 - index_of_refraction) / (1.0 + index_of_refraction);
    let r0_squared = r0 * r0;
    return r0_squared + (1.0 - r0_squared) * (1.0 - cosine).pow(5);
}

impl Material for Glass {
    fn scatter(&self, incoming_ray: Ray, intersection: &Intersection) -> (bool, Ray, Color) {
        let refraction_ratio = {
            if intersection.entering_material {
                1.0 / self.index_of_refraction
            } else {
                // otherwise leaving the material
                self.index_of_refraction / 1.0
            }
        };

        let normal = intersection.normal;

        let cosine_theta = cosine(-incoming_ray.d, normal);
        let sine_theta = (1.0 - cosine_theta * cosine_theta).sqrt();

        let cannot_refract = refraction_ratio * sine_theta > 1.0;
        let direction = if cannot_refract
            || reflectance(cosine_theta, refraction_ratio) > random_f32(0.0, 1.0)
        {
            incoming_ray.d.reflect(normal)
        } else {
            refract(incoming_ray.d, normal, refraction_ratio)
        };

        let scattered_ray = Ray::new(intersection.hit_point, direction);
        return (true, scattered_ray, Color::new(1.0, 1.0, 1.0));
    }

    fn is_specular(&self) -> bool {
        return true;
    }
}
