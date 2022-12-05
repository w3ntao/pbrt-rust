use crate::core::pbrt::*;

pub struct DebuggerScatterRay {
    world: Arc<Scene>,
}

impl DebuggerScatterRay {
    #[allow(dead_code)]
    pub fn new(_world: Arc<Scene>) -> Self {
        return Self { world: _world };
    }
}

impl Integrator for DebuggerScatterRay {
    fn get_radiance(&self, ray: Ray) -> Color {
        let mut interaction = SurfaceInteraction::default();
        if !self.world.intersect(&ray, &mut interaction) {
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
            )
        {
            return Color::black();
        }

        let direction = scattered_direction.normalize();
        let base: f32 = 10.0;
        let soft_max_direction = Vector3::new(
            base.powf(direction.x),
            base.powf(direction.y),
            base.powf(direction.z),
        )
        .normalize();

        return Color::new(
            soft_max_direction.x,
            soft_max_direction.y,
            soft_max_direction.z,
        );
    }
}
