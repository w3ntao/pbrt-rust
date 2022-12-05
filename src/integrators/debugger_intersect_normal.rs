use crate::core::pbrt::*;

pub struct DebuggerIntersectNormal {
    world: Arc<Scene>,
}

impl DebuggerIntersectNormal {
    #[allow(dead_code)]
    pub fn new(_world: Arc<Scene>) -> Self {
        return Self { world: _world };
    }
}

impl Integrator for DebuggerIntersectNormal {
    fn get_radiance(&self, ray: Ray) -> Color {
        let mut interaction = SurfaceInteraction::default();
        if !self.world.intersect(&ray, &mut interaction) {
            return Color::black();
        }

        let normal = interaction.n.normalize();
        return Color::new(normal.x.abs(), normal.y.abs(), normal.z.abs());
    }
}
