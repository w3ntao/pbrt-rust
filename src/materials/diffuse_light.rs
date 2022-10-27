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

    fn emit(&self, _emission: &mut Color, _interaction: &SurfaceInteraction) -> bool {
        *_emission = self
            .emission
            .get_color(_interaction.u, _interaction.v, _interaction.p);

        return true;
    }
}
