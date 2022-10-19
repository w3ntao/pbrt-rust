use crate::core::interfaces::*;

pub struct SolidColor {
    color: Color,
}

impl SolidColor {
    pub(crate) fn new(_color: Color) -> Self {
        SolidColor { color: _color }
    }
}

impl Texture for SolidColor {
    fn get_color(&self, _: f32, _: f32, _: Point) -> Color {
        return self.color;
    }
}
