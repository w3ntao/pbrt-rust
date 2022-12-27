use crate::core::pbrt::*;

pub struct DebuggerScatterRay {}

impl Default for DebuggerScatterRay {
    fn default() -> Self {
        return Self {};
    }
}

impl Integrator for DebuggerScatterRay {
    fn get_radiance(&self, ray: Ray, scene: Arc<Scene>, sampler: &mut dyn Sampler) -> Color {
        let mut interaction = SurfaceInteraction::default();
        if !scene.intersect(&ray, &mut interaction, sampler) {
            return Color::black();
        }

        let mut scattered_direction = Vector3::invalid();
        let mut attenuation = Color::black();
        if !interaction
            .material
            .as_ref()
            .expect("material is None")
            .scatter(
                ray,
                &interaction,
                &mut scattered_direction,
                &mut attenuation,
                sampler,
            )
        {
            return Color::black();
        }

        return scattered_direction.softmax_color();
    }
}
