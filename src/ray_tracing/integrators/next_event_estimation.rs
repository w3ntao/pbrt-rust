use std::sync::Arc;

use crate::fundamental::color::*;
use crate::fundamental::constants::INTERSECT_OFFSET;
use crate::fundamental::vector3::{cosine, dot};
use crate::ray_tracing::integrator::Integrator;
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
    fn trace(&self, ray: Ray, last_hit_specular: bool, depth: u32) -> Color {
        if depth == self.max_depth {
            return Color::black();
        }

        let intersection = self.world.intersect(&ray, INTERSECT_OFFSET, f32::INFINITY);
        // with INTERSECT_OFFSET, we can avoid the situation when the ray
        // re-hit the surface it just leave

        if !intersection.intersected() {
            return Color::black();
        }

        let emission = intersection.material.emit(&intersection);

        let (scattered, scattered_ray, attenuation) =
            intersection.material.scatter(ray, &intersection);
        if !scattered {
            // to avoid double sampling on light
            return if depth == 0 || last_hit_specular { emission } else { Color::black() };
        }

        if intersection.material.is_specular() {
            return self.trace(scattered_ray, true, depth + 1) * attenuation;
        }

        // for scattering: cos(theta) / PI cancel out for scattering_pdf and sample_pdf
        let indirect_illumination = self.trace(scattered_ray, false, depth + 1) * attenuation;

        let (light_id, light_point, light_normal, light_area) = self.world.sample_light();
        let towards_light = light_point - intersection.hit_point;
        let distance_squared = towards_light.length_squared();
        let towards_light = towards_light.normalize();

        // sampled light at the back side of object normal
        if dot(towards_light, intersection.normal) <= 0.0 {
            return emission + indirect_illumination;
        }

        let shadow_ray = Ray::new(intersection.hit_point, towards_light);

        // with light_cosine, the light emits unidirectionally
        let light_cosine = cosine(-towards_light, light_normal);
        if light_cosine <= 0.0 {
            return emission + indirect_illumination;
        }

        let shadow_intersection =
            self.world
                .intersect(&shadow_ray, INTERSECT_OFFSET, f32::INFINITY);

        // couldn't reach the sampled light
        if !shadow_intersection.intersected() || shadow_intersection.object_id != light_id {
            return emission + indirect_illumination;
        }

        let sample_light_pdf = distance_squared / (light_cosine * light_area);

        let direct_illumination = shadow_intersection.material.emit(&shadow_intersection)
            * attenuation
            * intersection.material.scattering_pdf(
            ray.direction,
            intersection.normal,
            towards_light,
        )
            / sample_light_pdf;

        return emission + direct_illumination + indirect_illumination;
    }
}

impl Integrator for NextEventEstimation {
    fn get_radiance(&self, ray: Ray) -> Color {
        return self.trace(ray, false, 0);
    }
}
