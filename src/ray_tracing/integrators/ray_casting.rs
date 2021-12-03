use crate::fundamental::vector::*;
use crate::ray_tracing::ray::*;
use crate::ray_tracing::group::*;

pub struct RayCastingIntegrator {
    world: Group,
}

impl RayCastingIntegrator {
    pub fn new(_world: Group) -> Self {
        return Self { world: _world };
    }

    pub fn get_radiance(&self, ray: &Ray) -> Vector {
        let intersect = self.world.intersect(ray, f32::INFINITY);
        if !intersect.intersected() {
            return Vector::zero();
        }

        let normal = intersect.normal.normalize();
        let grey = 0.0_f32.max(dot(-ray.direction, normal));
        return Vector::new(grey, grey, grey);
    }
}
