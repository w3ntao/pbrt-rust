use crate::core::pbrt::*;

pub struct DiffuseLight {
    emission: Color,
}

impl DiffuseLight {
    pub fn new(_emission: Color) -> DiffuseLight {
        DiffuseLight {
            emission: _emission,
        }
    }
}

impl Material for DiffuseLight {
    fn scatter(&self, _: Ray, _: &SurfaceInteraction) -> (bool, Ray, Color) {
        return (false, Ray::dummy(), Color::black());
    }

    fn emit(&self, _emission: &mut Color) -> bool {
        *_emission = self.emission;
        return true;
    }
}
