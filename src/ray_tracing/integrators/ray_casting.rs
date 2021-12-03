use crate::fundamental::vector::*;
use crate::ray_tracing::ray::Ray;
use crate::ray_tracing::group::*;
use crate::ray_tracing::integrator_trait::Integrator;

pub struct RayCastingIntegrator {
    world: Group,
}

impl RayCastingIntegrator {
    pub fn new(_world: Group) -> Self {
        return Self { world: _world };
    }
}

impl Integrator for RayCastingIntegrator {
    fn get_radiance(&self, ray: &Ray) -> Vector {
        let intersect = self.world.intersect(ray, f32::INFINITY);
        if !intersect.intersected() {
            return Vector::zero();
        }

        let normal = intersect.normal.normalize();
        let grey = 0.0_f32.max(dot(-ray.direction, normal));
        return Vector::new(grey, grey, grey);
    }
}
