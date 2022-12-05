use crate::core::pbrt::*;

pub struct DebuggerRayCastingDotNormal {
    world: Arc<Scene>,
}

impl DebuggerRayCastingDotNormal {
    pub fn new(_world: Arc<Scene>) -> Self {
        return Self { world: _world };
    }
}

impl Integrator for DebuggerRayCastingDotNormal {
    fn get_radiance(&self, ray: Ray) -> Color {
        let mut interaction = SurfaceInteraction::default();

        if !self.world.intersect(&ray, &mut interaction) {
            return Color::black();
        }

        let normal = interaction.n.normalize();
        let grey = normal.dot(-ray.d).max(0.0);
        return Color::new(grey, grey, grey);
    }
}
