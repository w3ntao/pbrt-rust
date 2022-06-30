use std::sync::Arc;

use crate::fundamental::color::*;
use crate::fundamental::constants::INTERSECT_OFFSET;
use crate::fundamental::vector3::cosine;
use crate::ray_tracing::integrator::Integrator;
use crate::ray_tracing::ray::Ray;
use crate::ray_tracing::world::World;

pub struct NextEventEstimation {
    world: Arc<World>,
}

impl NextEventEstimation {
    pub fn new(_world: Arc<World>) -> Self {
        return Self {
            world: _world,
        };
    }
}

impl NextEventEstimation {
    fn trace(&self, ray: Ray, depth: u32) -> Color {
        //TODO: currently works for direct illumination only

        if depth == 0 {
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

        let (light_id, light_point, light_normal, light_area) = self.world.sample_light();
        let towards_light = (light_point - intersection.hit_point).normalize();

        let shadow_ray = Ray::new(intersection.hit_point, towards_light);
        let shadow_intersection = self.world.intersect(&shadow_ray, INTERSECT_OFFSET, f32::INFINITY);

        if !shadow_intersection.intersected() || shadow_intersection.object_id != light_id {
            return Color::black();
        }

        // with light_cosine, the light emits unidirectionally 
        let light_cosine = cosine(-towards_light, light_normal);
        if light_cosine <= 0.0 {
            return Color::black();
        }

        let scattering_pdf = intersection.material.scattering_pdf(ray.direction, intersection.normal, towards_light);
        if scattering_pdf <= 0.0 {
            return Color::black();
        }

        let sample_pdf = shadow_intersection.distance * shadow_intersection.distance / (light_cosine * light_area);

        return attenuation
            * shadow_intersection.material.emit(&shadow_intersection)
            * scattering_pdf
            / sample_pdf;
    }
}

impl Integrator for NextEventEstimation {
    fn get_radiance(&self, ray: Ray) -> Color {
        return self.trace(ray, 50);
    }
}
