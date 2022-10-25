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

    fn emit(&self, intersection: &SurfaceInteraction) -> Color {
        return self
            .emission
            .get_color(intersection.u, intersection.v, intersection.hit_point);
    }
}
