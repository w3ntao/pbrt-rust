use std::sync::Arc;

use crate::fundamental::color::Color;
use crate::fundamental::point::Point;
use crate::ray_tracing::texture::Texture;
use crate::ray_tracing::textures::perlin::Perlin;

pub struct NoiseTexture {
    noise: Perlin,
}

impl NoiseTexture {
    pub fn new() -> NoiseTexture {
        NoiseTexture {
            noise: Perlin::new(),
        }
    }
}

impl Texture for NoiseTexture {
    fn get_color(&self, _: f32, _: f32, point: Point) -> Color {
        return Color::new(1.0, 1.0, 1.0) * self.noise.noise(point);
    }
}
