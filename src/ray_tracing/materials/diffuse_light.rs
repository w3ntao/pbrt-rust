use std::sync::Arc;

use crate::fundamental::color::Color;
use crate::ray_tracing::intersection::Intersection;
use crate::ray_tracing::material::Material;
use crate::ray_tracing::ray::Ray;
use crate::ray_tracing::texture::Texture;

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
        panic!("scatter() is not implemented for DiffuseLight");
    }

    fn emit(&self, intersection: &Intersection) -> Color {
        return self
            .emission
            .get_color(intersection.u, intersection.v, intersection.hit_point);
    }
}
