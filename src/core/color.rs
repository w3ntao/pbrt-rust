use crate::core::pbrt::*;
use std::fmt::Formatter;

#[derive(Copy, Clone)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
}

impl Color {
    pub fn new(_r: f32, _g: f32, _b: f32) -> Self {
        return Self {
            r: _r,
            g: _g,
            b: _b,
        };
    }

    pub fn black() -> Self {
        return Color::new(0.0, 0.0, 0.0);
    }

    pub fn is_finite(&self) -> bool {
        let finite = |x: f32| -> bool { x.is_finite() };
        return finite(self.b) && finite(self.g) && finite(self.b);
    }

    pub fn max_component(&self) -> f32 {
        return self.r.max(self.g).max(self.b);
    }
}

impl std::fmt::Display for Color {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {}, {})", self.r, self.g, self.b)
    }
}

impl ops::Add<Color> for Color {
    type Output = Color;
    fn add(self, rhs: Color) -> Color {
        return Color {
            r: self.r + rhs.r,
            g: self.g + rhs.g,
            b: self.b + rhs.b,
        };
    }
}

impl ops::Mul<f32> for Color {
    type Output = Color;
    fn mul(self, factor: f32) -> Color {
        return Color {
            r: self.r * factor,
            g: self.g * factor,
            b: self.b * factor,
        };
    }
}

impl ops::Mul<Color> for f32 {
    type Output = Color;
    fn mul(self, rhs: Color) -> Color {
        return rhs * self;
    }
}

impl ops::Mul<Color> for Color {
    type Output = Color;
    fn mul(self, rhs: Color) -> Color {
        return Color {
            r: self.r * rhs.r,
            g: self.g * rhs.g,
            b: self.b * rhs.b,
        };
    }
}

impl ops::Div<f32> for Color {
    type Output = Color;
    fn div(self, divisor: f32) -> Color {
        let inv = 1.0 / divisor;
        return Color {
            r: self.r * inv,
            g: self.g * inv,
            b: self.b * inv,
        };
    }
}

impl ops::AddAssign<Color> for Color {
    fn add_assign(&mut self, rhs: Color) {
        self.r += rhs.r;
        self.g += rhs.g;
        self.b += rhs.b;
    }
}

impl ops::MulAssign<Color> for Color {
    fn mul_assign(&mut self, rhs: Color) {
        self.r *= rhs.r;
        self.g *= rhs.g;
        self.b *= rhs.b;
    }
}

impl ops::DivAssign<f32> for Color {
    fn div_assign(&mut self, divisor: f32) {
        let inv = 1.0 / divisor;
        self.r *= inv;
        self.g *= inv;
        self.b *= inv;
    }
}
