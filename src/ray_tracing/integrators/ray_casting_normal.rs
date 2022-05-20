use std::sync::Arc;

use crate::fundamental::color::*;
use crate::ray_tracing::integrator::Integrator;
use crate::ray_tracing::ray::Ray;
use crate::ray_tracing::world::World;

pub struct RayCastingNormal {
    world: Arc<World>,
}

impl RayCastingNormal {
    pub fn new(_world: Arc<World>) -> Self {
        return Self { world: _world };
    }
}

impl Integrator for RayCastingNormal {
    fn get_radiance(&self, ray: Ray) -> Color {
        let intersect = self.world.scene.intersect(&ray, 0.0, f32::INFINITY);
        if !intersect.intersected() {
            return Color::black();
        }

        let normal = intersect.normal.normalize();
        return Color::new(normal.x, normal.y, normal.z);
    }
}
