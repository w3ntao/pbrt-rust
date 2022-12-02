use crate::core::pbrt::*;

pub struct DebuggerScatterRay {
    world: Arc<World>,
}

impl DebuggerScatterRay {
    #[allow(dead_code)]
    pub fn new(_world: Arc<World>) -> Self {
        return Self { world: _world };
    }
}

impl Integrator for DebuggerScatterRay {
    fn get_radiance(&self, ray: Ray) -> Color {
        let background = Color::new(0.7, 0.8, 1.0);
        let mut interaction = SurfaceInteraction::default();
        if !self.world.intersect(&ray, &mut interaction) {
            return background;
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
            return background;
        }

        scattered_direction = scattered_direction.normalize();
        return Color::new(
            scattered_direction.x,
            scattered_direction.y,
            scattered_direction.z,
        );
    }
}
