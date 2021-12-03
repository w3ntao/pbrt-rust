#[derive(Copy, Clone)]
pub struct RGBColor {
    pub r: f32,
    pub g: f32,
    pub b: f32,
}

impl RGBColor {
    pub fn new(_r: f32, _g: f32, _b: f32) -> Self {
        return Self {
            r: _r,
            g: _g,
            b: _b,
        };
    }

    pub fn black() -> Self {
        return RGBColor::new(0.0, 0.0, 0.0);
    }
}
