use crate::fundamental::color::Color;
use crate::fundamental::point::Point;

pub trait Texture: Send + Sync {
    fn get_color(&self, u: f32, v: f32, point: Point) -> Color;
}
