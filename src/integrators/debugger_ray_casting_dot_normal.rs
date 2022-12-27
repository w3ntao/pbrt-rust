use crate::core::pbrt::*;

pub struct DebuggerRayCastingDotNormal {}

impl Default for DebuggerRayCastingDotNormal {
    fn default() -> Self {
        return Self {};
    }
}

impl Integrator for DebuggerRayCastingDotNormal {
    fn get_radiance(&self, ray: Ray, scene: Arc<Scene>, sampler: &mut dyn Sampler) -> Color {
        let mut interaction = SurfaceInteraction::default();

        if !scene.intersect(&ray, &mut interaction, sampler) {
            return Color::black();
        }

        let normal = interaction.n.normalize();
        let grey = normal.dot(-ray.d).max(0.0);
        return Color::new(grey, grey, grey);
    }
}
