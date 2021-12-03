use crate::fundamental::vector::*;
use crate::ray_tracing::ray::Ray;
use crate::ray_tracing::integrator_trait::Integrator;
use crate::ray_tracing::world::World;

pub struct RayCastingIntegrator<'a> {
    world: &'a World<'a>,
}

impl<'a> RayCastingIntegrator<'a> {
    pub fn new(_world: &'a World<'a>) -> Self {
        return Self { world: _world };
    }
}

impl<'a> Integrator for RayCastingIntegrator<'a> {
    fn get_radiance(&self, ray: &Ray) -> Vector {
        let intersect = self.world.scene.intersect(ray, f32::INFINITY);
        if !intersect.intersected() {
            return Vector::zero();
        }

        let normal = intersect.normal.normalize();
        let grey = 0.0_f32.max(dot(-ray.direction, normal));
        return Vector::new(grey, grey, grey);
    }
}
