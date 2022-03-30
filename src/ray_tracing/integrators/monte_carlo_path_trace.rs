use std::sync::Arc;

use crate::fundamental::color::*;
use crate::fundamental::constants::INTERSECT_OFFSET;
use crate::ray_tracing::integrator::Integrator;
use crate::ray_tracing::ray::Ray;
use crate::ray_tracing::world::World;

pub struct MonteCarloPathTrace {
    world: Arc<World>,
}

impl MonteCarloPathTrace {
    pub fn new(_world: Arc<World>) -> Self {
        return Self { world: _world };
    }
}

impl MonteCarloPathTrace {
    fn trace(&self, ray: &Ray, depth: i32) -> Color {
        if depth <= 0 {
            return Color::black();
        }

        let intersection = self.world.scene.intersect(ray, INTERSECT_OFFSET, f32::INFINITY);
        // with INTERSECT_OFFSET, we can avoid the situation when the ray
        // re-hit the surface it just leave

        if intersection.intersected() {
            let mut scattered_ray = Ray::dummy();
            let attenuation = intersection.material.scatter(&ray, &intersection, &mut scattered_ray);
            return attenuation * self.trace(&scattered_ray, depth - 1);
        }

        // shoot into sky
        let unit_direction = ray.direction.normalize();
        let t = 0.5 * (unit_direction.y + 1.0);
        return (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0);
    }
}

impl Integrator for MonteCarloPathTrace {
    fn get_radiance(&self, ray: &Ray) -> Color {
        return self.trace(ray, 50);
    }
}
