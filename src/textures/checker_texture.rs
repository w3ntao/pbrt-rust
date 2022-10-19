use crate::core::pbrt::*;

pub struct CheckerTexture {
    even: Arc<dyn Texture>,
    odd: Arc<dyn Texture>,
}

impl CheckerTexture {
    pub fn new(_odd: Arc<dyn Texture>, _even: Arc<dyn Texture>) -> CheckerTexture {
        CheckerTexture {
            odd: _odd,
            even: _even,
        }
    }
}

impl Texture for CheckerTexture {
    fn get_color(&self, u: f32, v: f32, point: Point) -> Color {
        let sine_val = (point.x * 10.0).sin() * (point.y * 10.0).sin() * (point.z * 10.0).sin();

        return if sine_val < 0.0 {
            self.odd.get_color(u, v, point)
        } else {
            self.even.get_color(u, v, point)
        };
    }
}
