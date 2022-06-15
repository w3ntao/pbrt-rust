use std::sync::Arc;

use crate::fundamental::color::Color;
use crate::fundamental::point::Point;
use crate::ray_tracing::intersection::Intersection;
use crate::ray_tracing::material::Material;
use crate::ray_tracing::ray::Ray;
use crate::ray_tracing::texture::Texture;

pub struct DiffuseLight {
    emission: Arc<dyn Texture>,
}

impl DiffuseLight {
    pub fn new(_texture: Arc<dyn Texture>) -> DiffuseLight {
        DiffuseLight {
            emission: _texture,
        }
    }
}

impl Material for DiffuseLight {
    fn scatter(&self, _: Ray, _: &Intersection) -> (bool, Ray, Color) {
        return (false, Ray::dummy(), Color::black());
    }

    fn emit(&self, u: f32, v: f32, point: Point) -> Color {
        return self.emission.get_color(u, v, point);
    }
}
