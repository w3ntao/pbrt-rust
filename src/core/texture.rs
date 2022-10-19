use crate::core::color::Color;
use crate::core::point::Point;

pub trait Texture: Send + Sync {
    fn get_color(&self, u: f32, v: f32, point: Point) -> Color;
}
