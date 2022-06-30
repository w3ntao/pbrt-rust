use std::sync::Arc;

use crate::fundamental::color::*;
use crate::fundamental::constants::INTERSECT_OFFSET;
use crate::fundamental::vector3::{cosine, dot};
use crate::ray_tracing::integrator::Integrator;
use crate::ray_tracing::ray::Ray;
use crate::ray_tracing::world::World;

pub struct SampleLight {
    world: Arc<World>,
}

impl SampleLight {
    pub fn new(_world: Arc<World>) -> Self {
        return Self { world: _world };
    }
}

impl SampleLight {
    fn trace(&self, ray: Ray, depth: u32, max_depth: u32) -> Color {
        if depth == max_depth {
            return Color::black();
        }

        let intersection = self.world.intersect(&ray, INTERSECT_OFFSET, f32::INFINITY);
        // with INTERSECT_OFFSET, we can avoid the situation when the ray
        // re-hit the surface it just leave

        if !intersection.intersected() {
            return Color::black();
        }

        let emission = intersection.material.emit(&intersection);

        let (scattered, _, attenuation) = intersection.material.scatter(ray, &intersection);
        if !scattered {
            return emission;
        }

        let (_, light_point, light_normal, light_area) = self.world.sample_light();
        let towards_light = light_point - intersection.hit_point;
        if dot(towards_light, intersection.normal) <= 0.0 {
            return emission;
        }

        // with light_cosine, the light emits unidirectionally
        let light_cosine = cosine(-towards_light, light_normal);
        if light_cosine < 0.0 {
            return emission;
        }

        let distance_squared = towards_light.length_squared();
        let towards_light = towards_light.normalize();

        let shadow_ray = Ray::new(intersection.hit_point, towards_light);

        let sample_light_pdf = distance_squared / (light_cosine * light_area);

        return emission
            + self.trace(shadow_ray, depth + 1, max_depth)
                * attenuation
                * intersection.material.scattering_pdf(
                    ray.direction,
                    intersection.normal,
                    towards_light,
                )
                / sample_light_pdf;
    }
}

impl Integrator for SampleLight {
    fn get_radiance(&self, ray: Ray) -> Color {
        return self.trace(ray, 0, 50);
    }
}
