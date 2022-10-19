use crate::core::pbrt::*;
use crate::textures::perlin::Perlin;

pub struct NoiseTexture {
    noise: Perlin,
    scale: f32,
}

impl NoiseTexture {
    pub fn new(_scale: f32) -> NoiseTexture {
        NoiseTexture {
            noise: Perlin::new(),
            scale: _scale,
        }
    }
}

impl Texture for NoiseTexture {
    fn get_color(&self, _: f32, _: f32, point: Point) -> Color {
        return Color::new(1.0, 1.0, 1.0)
            * 0.5
            * (1.0 + (self.scale * point.z + 10.0 * self.noise.turbulence(point, 7)).sin());
    }
}
