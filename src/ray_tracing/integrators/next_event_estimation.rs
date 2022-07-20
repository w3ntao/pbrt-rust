use std::sync::Arc;

use crate::fundamental::color::*;
use crate::fundamental::constants::INTERSECT_OFFSET;
use crate::fundamental::vector3::{cosine, dot};
use crate::ray_tracing::integrator::Integrator;
use crate::ray_tracing::intersection::Intersection;
use crate::ray_tracing::ray::Ray;
use crate::ray_tracing::world::World;

pub struct NextEventEstimation {
    world: Arc<World>,
    max_depth: u32,
}

impl NextEventEstimation {
    pub fn new(_world: Arc<World>) -> Self {
        return Self {
            world: _world,
            max_depth: 50,
        };
    }
}

impl NextEventEstimation {
    fn get_direct_illumination(&self, intersection: &Intersection, ray: &Ray) -> Color {
        let (light_id, light_point, light_normal, light_area) = self.world.sample_light();
        let towards_light = light_point - intersection.hit_point;
        let distance_squared = towards_light.length_squared();
        let towards_light = towards_light.normalize();

        // sampled light at the back side of object normal
        if dot(towards_light, intersection.normal) <= 0.0 {
            return Color::black();
        }

        let shadow_ray = Ray::new(intersection.hit_point, towards_light);

        // with light_cosine, the light emits uni-directionally
        let light_cosine = cosine(-towards_light, light_normal);
        if light_cosine <= 0.0 {
            return Color::black();
        }

        let shadow_intersection =
            self.world
                .intersect(&shadow_ray, INTERSECT_OFFSET, f32::INFINITY);

        // couldn't reach the sampled light
        if !shadow_intersection.intersected() || shadow_intersection.object_id != light_id {
            return Color::black();
        }

        let sample_light_pdf = distance_squared / (light_cosine * light_area);

        return shadow_intersection.material.emit(&shadow_intersection)
            * intersection.material.scattering_pdf(
                ray.direction,
                intersection.normal,
                towards_light,
            )
            / sample_light_pdf;
    }

    fn trace(&self, ray: Ray) -> Color {
        let mut radiance = Color::black();
        let mut throughput = Color::new(1.0, 1.0, 1.0);
        let mut ray = ray;
        let mut last_hit_specular = false;

        for depth in 0..self.max_depth {
            let intersection = self.world.intersect(&ray, INTERSECT_OFFSET, f32::INFINITY);
            // with INTERSECT_OFFSET, we can avoid the situation when the ray
            // re-hit the surface it just leave

            if !intersection.intersected() {
                break;
            }

            let emission = intersection.material.emit(&intersection);

            let (scattered, scattered_ray, attenuation) =
                intersection.material.scatter(ray, &intersection);
            if !scattered {
                if depth == 0 || last_hit_specular {
                    radiance += throughput * emission;
                }
                break;
            }

            radiance += throughput * emission;

            last_hit_specular = intersection.material.is_specular();
            if !last_hit_specular {
                radiance +=
                    throughput * attenuation * self.get_direct_illumination(&intersection, &ray);
            }

            throughput *= attenuation;
            ray = scattered_ray;
        }

        return radiance;
    }
}

impl Integrator for NextEventEstimation {
    fn get_radiance(&self, ray: Ray) -> Color {
        return self.trace(ray);
    }
}
