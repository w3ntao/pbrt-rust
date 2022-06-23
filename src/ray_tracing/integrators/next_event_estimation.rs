use std::sync::Arc;

use rand_distr::num_traits::pow;

use crate::fundamental::color::*;
use crate::fundamental::constants::INTERSECT_OFFSET;
use crate::fundamental::utility::*;
use crate::fundamental::vector3::cosine;
use crate::ray_tracing::integrator::Integrator;
use crate::ray_tracing::ray::Ray;
use crate::ray_tracing::world::World;

pub struct NextEventEstimation {
    world: Arc<World>,
    background: Color,
}

impl NextEventEstimation {
    pub fn new(_world: Arc<World>, _background: Color) -> Self {
        return Self {
            world: _world,
            background: _background,
        };
    }
}

impl NextEventEstimation {
    fn trace(&self, ray: Ray, depth: u32) -> Color {
        if depth == 0 {
            return Color::black();
        }

        let intersection = self.world.intersect(&ray, INTERSECT_OFFSET, f32::INFINITY);
        // with INTERSECT_OFFSET, we can avoid the situation when the ray
        // re-hit the surface it just leave

        if !intersection.intersected() {
            return Color::black();
        }

        let emission = intersection.material.emit(intersection.u, intersection.v, intersection.hit_point);

        let (scattered, scattered_ray, attenuation) = intersection.material.scatter(ray, &intersection);

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

        let light_cosine   = cosine(-towards_light, light_normal).abs();
        if light_cosine <= 0.0 {
            return Color::black();
        }

        let surface_cosine = cosine(towards_light, intersection.normal);
        if surface_cosine <= 0.0 {
            return Color::black();
        }

        //TODO: move scattering_pdf into Material
        let scattering_pdf = surface_cosine / PI;

        let pdf = shadow_intersection.distance * shadow_intersection.distance / (light_cosine * light_area);

        let direct_lighting = shadow_intersection.material.emit(shadow_intersection.u, shadow_intersection.v, shadow_intersection.hit_point);

        return attenuation * direct_lighting * scattering_pdf / pdf;
    }
}

impl Integrator for NextEventEstimation {
    fn get_radiance(&self, ray: Ray) -> Color {
        return self.trace(ray, 50);
    }
}
