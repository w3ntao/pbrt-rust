use crate::core::pbrt::*;

pub struct DebuggerIntersectNormal {}

impl Default for DebuggerIntersectNormal {
    fn default() -> Self {
        return Self {};
    }
}

impl Integrator for DebuggerIntersectNormal {
    fn get_radiance(&self, ray: Ray, scene: Arc<Scene>) -> Color {
        let mut interaction = SurfaceInteraction::default();
        if !scene.intersect(&ray, &mut interaction) {
            return Color::black();
        }

        let normal = interaction.n.normalize();
        return Color::new(normal.x.abs(), normal.y.abs(), normal.z.abs());
    }
}
