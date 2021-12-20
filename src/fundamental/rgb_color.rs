use std::ops;

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

impl ops::Add<RGBColor> for RGBColor {
    type Output = RGBColor;
    fn add(self, rhs: RGBColor) -> RGBColor {
        return RGBColor {
            r: self.r + rhs.r,
            g: self.g + rhs.g,
            b: self.b + rhs.b,
        };
    }
}

impl ops::Mul<f32> for RGBColor {
    type Output = RGBColor;
    fn mul(self, rhs: f32) -> RGBColor {
        return RGBColor {
            r: self.r * rhs,
            g: self.g * rhs,
            b: self.b * rhs,
        };
    }
}

impl ops::Mul<RGBColor> for f32 {
    type Output = RGBColor;
    fn mul(self, rhs: RGBColor) -> RGBColor {
        return rhs * self;
    }
}

impl ops::Mul<RGBColor> for RGBColor {
    type Output = RGBColor;
    fn mul(self, rhs: RGBColor) -> RGBColor {
        return RGBColor {
            r: self.r * rhs.r,
            g: self.g * rhs.g,
            b: self.b * rhs.b,
        };
    }
}
