use std::sync::Arc;
use crate::fundamental::vector::*;
use crate::fundamental::rgb_color::*;
use crate::ray_tracing::ray::Ray;
use crate::ray_tracing::integrator::Integrator;
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
    fn ray_color(&self, ray: &Ray, depth: i32) -> RGBColor {
        if depth <= 0 {
            return RGBColor::black();
        }

        let intersect = self.world.scene.intersect(ray, f32::INFINITY);
        if intersect.intersected() {
            let mut attenuation = RGBColor::black();
            let mut scatter_ray = Ray::dummy();

            if intersect.material.scatter(&mut attenuation, &mut scatter_ray, &ray, &intersect) {
                return attenuation * self.ray_color(&scatter_ray, depth - 1);
            }

            return RGBColor::black();
        }

        let unit_direction = ray.direction.normalize();
        let t = 0.5 * (unit_direction.y + 1.0);
        return (1.0 - t) * RGBColor::new(1.0, 1.0, 1.0) + t * RGBColor::new(0.5, 0.7, 1.0);
    }
}

impl Integrator for MonteCarloPathTrace {
    fn get_radiance(&self, ray: &Ray) -> RGBColor {
        return self.ray_color(ray, 20);
    }
}
