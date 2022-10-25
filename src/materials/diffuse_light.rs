use crate::core::pbrt::*;

pub struct DiffuseLight {
    emission: Arc<dyn Texture>,
}

impl DiffuseLight {
    pub fn new(_texture: Arc<dyn Texture>) -> DiffuseLight {
        DiffuseLight { emission: _texture }
    }
}

impl Material for DiffuseLight {
    fn scatter(&self, _: Ray, _: &SurfaceInteraction) -> (bool, Ray, Color) {
        return (false, Ray::dummy(), Color::black());
    }

    fn emit(&self, surface_interaction: &SurfaceInteraction) -> Color {
        return self.emission.get_color(
            surface_interaction.u,
            surface_interaction.v,
            surface_interaction.p,
        );
    }
}
