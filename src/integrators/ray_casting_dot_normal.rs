use crate::core::pbrt::*;

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
        let surface_interaction = self.world.intersect(&ray, 0.0, f32::INFINITY);
        if !surface_interaction.intersected() {
            return Color::black();
        }

        let normal = surface_interaction.n.normalize();
        let grey = normal.dot(-ray.d).max(0.0);
        return Color::new(grey, grey, grey);
    }
}
