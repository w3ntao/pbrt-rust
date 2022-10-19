use crate::core::integrator::Integrator;
use crate::core::ray::Ray;
use crate::core::world::World;
use crate::fundamental::color::*;
use crate::fundamental::vector3::*;
use std::sync::Arc;

pub struct RayCastingDotNormal {
    world: Arc<World>,
}

impl RayCastingDotNormal {
    pub fn new(_world: Arc<World>) -> Self {
        return Self { world: _world };
    }
}

impl Integrator for RayCastingDotNormal {
    fn get_radiance(&self, ray: Ray) -> Color {
        let intersect = self.world.intersect(&ray, 0.0, f32::INFINITY);
        if !intersect.intersected() {
            return Color::black();
        }

        let normal = intersect.normal.normalize();
        let grey = dot(-ray.d, normal).max(0.0);
        return Color::new(grey, grey, grey);
    }
}
