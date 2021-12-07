use std::rc::Rc;
use crate::fundamental::vector::*;
use crate::fundamental::rgb_color::*;
use crate::ray_tracing::ray::Ray;
use crate::ray_tracing::integrator::Integrator;
use crate::ray_tracing::world::World;

pub struct RayCastingIntegrator {
    world: Rc<World>,
}

impl RayCastingIntegrator {
    pub fn new(_world: Rc<World>) -> Self {
        return Self { world: _world };
    }
}

impl Integrator for RayCastingIntegrator {
    fn get_radiance(&self, ray: Rc<Ray>) -> RGBColor {
        let intersect = self.world.scene.intersect(Rc::clone(&ray), f32::INFINITY);
        if !intersect.intersected() {
            return RGBColor::black();
        }

        let normal = intersect.normal.normalize();
        let grey = 0.0_f32.max(dot(-ray.direction, normal));
        return RGBColor::new(grey, grey, grey);
    }
}
