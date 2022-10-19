use crate::core::interfaces::*;

pub trait Texture: Send + Sync {
    fn get_color(&self, u: f32, v: f32, point: Point) -> Color;
}
