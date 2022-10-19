use crate::core::intersection::Intersection;
use crate::core::material::Material;
use crate::core::ray::Ray;
use crate::core::texture::Texture;
use crate::fundamental::color::Color;
use std::sync::Arc;

pub struct DiffuseLight {
    emission: Arc<dyn Texture>,
}

impl DiffuseLight {
    pub fn new(_texture: Arc<dyn Texture>) -> DiffuseLight {
        DiffuseLight { emission: _texture }
    }
}

impl Material for DiffuseLight {
    fn scatter(&self, _: Ray, _: &Intersection) -> (bool, Ray, Color) {
        return (false, Ray::dummy(), Color::black());
    }

    fn emit(&self, intersection: &Intersection) -> Color {
        return self
            .emission
            .get_color(intersection.u, intersection.v, intersection.hit_point);
    }
}
