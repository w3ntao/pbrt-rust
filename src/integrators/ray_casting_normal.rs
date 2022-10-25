use crate::core::pbrt::*;

pub struct RayCastingNormal {
    world: Arc<World>,
}

impl RayCastingNormal {
    #[allow(dead_code)]
    pub fn new(_world: Arc<World>) -> Self {
        return Self { world: _world };
    }
}

impl Integrator for RayCastingNormal {
    fn get_radiance(&self, ray: Ray) -> Color {
        let surface_interaction = self.world.intersect(&ray, 0.0, f32::INFINITY);
        if !surface_interaction.intersected() {
            return Color::black();
        }

        let normal = surface_interaction.n.normalize();
        return Color::new(normal.x.abs(), normal.y.abs(), normal.z.abs());
    }
}
